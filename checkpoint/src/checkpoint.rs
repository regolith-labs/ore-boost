use std::sync::Arc;

use anyhow::Result;
use ore_boost_api::state::Stake;
use ore_boost_api::{consts::CHECKPOINT_INTERVAL, state::Checkpoint};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer};

use crate::client::{AsyncClient, Client};
use crate::error::Error::ClockStillTicking;
use crate::{lookup_tables, notifier};

const MAX_ACCOUNTS_PER_TX: usize = 20;
const MAX_ATTEMPTS: usize = 10;

pub async fn run_all(client: Arc<Client>) -> Result<()> {
    // fetch all boosts
    let boosts = client.rpc.get_boosts().await?;
    // spawn task for each boost checkpoint
    let mut handles = vec![];
    for b in boosts {
        let client = Arc::clone(&client);
        let handle = tokio::spawn(async move {
            if let Err(err) = run(client.as_ref(), &b.mint).await {
                // log error then return
                let (pda, _) = ore_boost_api::state::boost_pda(b.mint);
                log::error!("{} -- exit", pda);
                return Err(err);
            }
            Ok::<_, anyhow::Error>(())
        });
        handles.push(handle);
    }
    // await checkpoints
    // early exit all tasks if one returns err
    futures::future::try_join_all(handles).await?;
    Ok(())
}

pub async fn run(client: &Client, mint: &Pubkey) -> Result<()> {
    // derive address
    let (boost_pda, _) = ore_boost_api::state::boost_pda(*mint);
    let (checkpoint_pda, _) = ore_boost_api::state::checkpoint_pda(boost_pda);
    // get accounts
    let mut total_stakers = 0;
    let _boost = client.rpc.get_boost(&boost_pda).await?;
    let mut checkpoint = client.rpc.get_checkpoint(&checkpoint_pda).await?;
    // sync lookup tables
    let mut stake_accounts = client.rpc.get_boost_stake_accounts(&boost_pda).await?;
    let mut lookup_tables =
        lookup_tables::sync(client, &boost_pda, stake_accounts.as_slice()).await?;
    // start checkpoint loop
    let mut attempt = 0;
    loop {
        log::info!("///////////////////////////////////////////////////////////");
        log::info!("// checkpoint");
        log::info!("{} -- {:?}", boost_pda, checkpoint);
        log::info!("{} -- attempt: {}", boost_pda, attempt);
        // notify admin if worker is stalling
        if attempt.eq(&MAX_ATTEMPTS) {
            attempt = 0;
            if let Err(err) = notifier::notify().await {
                log::info!("{:?} -- {:?}", boost_pda, err);
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue;
            }
        }
        // fetch boost for total stakers count
        match client.rpc.get_boost(&boost_pda).await {
            Ok(boost) => {
                // check for new stakers
                if boost.total_stakers.gt(&total_stakers) {
                    log::info!("{} -- found new stakers", boost_pda);
                    // fetch stake accounts
                    match client.rpc.get_boost_stake_accounts(&boost_pda).await {
                        Ok(sa) => {
                            stake_accounts = sa;
                            total_stakers = boost.total_stakers;
                        }
                        Err(err) => {
                            log::error!("{:?} -- {:?}", boost_pda, err);
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            attempt += 1;
                            continue;
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("{:?} -- {:?}", boost_pda, err);
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                attempt += 1;
                continue;
            }
        }
        // fetch checkpoint
        match client.rpc.get_checkpoint(&checkpoint_pda).await {
            Ok(cp) => {
                // always update checkpoint regardless of new timestamp
                // because the current-id may have moved
                checkpoint = cp;
                // if new checkpoint, sync lookup tables
                if cp.ts.ne(&checkpoint.ts) {
                    // sync lookup tables
                    match lookup_tables::sync(client, &boost_pda, stake_accounts.as_slice()).await {
                        Ok(luts) => {
                            lookup_tables = luts;
                            attempt = 0;
                        }
                        Err(err) => {
                            log::error!("{:?} -- {:?}", boost_pda, err);
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            attempt += 1;
                            continue;
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("{:?} -- {:?}", boost_pda, err);
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                attempt += 1;
                continue;
            }
        }
        // check for time
        if let Err(err) = check_for_time(client, &checkpoint, &boost_pda).await {
            // time has not elapsed or error
            log::info!("{:?} -- {:?}", boost_pda, err);
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            continue;
        }
        // filter stake accounts
        // against the checkpoint current-id,
        // recovering from a partial checkpoint if necessary
        let remaining_stake_accounts =
            remaining_stake_accounts(stake_accounts.as_mut_slice(), &checkpoint, &boost_pda);
        // rebase all stake accounts
        match rebase_all(
            client,
            mint,
            &boost_pda,
            remaining_stake_accounts.as_slice(),
            lookup_tables.as_slice(),
        )
        .await
        {
            Ok(()) => {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
            Err(err) => {
                log::info!("{:?} -- {:?}", boost_pda, err);
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                attempt += 1;
            }
        }
    }
}

/// sort then filter stake accounts against checkpoint current-id
fn remaining_stake_accounts(
    stake_accounts: &mut [(Pubkey, Stake)],
    checkpoint: &Checkpoint,
    boost_pda: &Pubkey,
) -> Vec<Pubkey> {
    // sort by stake id
    stake_accounts.sort_by(|(_, left), (_, right)| left.id.cmp(&right.id));
    // filter for remaining
    let remaining_accounts: Vec<_> = stake_accounts
        .iter()
        .filter_map(|(pubkey, stake)| {
            if stake.id >= checkpoint.current_id {
                Some(*pubkey)
            } else {
                None
            }
        })
        .collect();
    log::info!(
        "{:?} -- checkpoint current id: {:?}",
        boost_pda,
        checkpoint.current_id
    );
    log::info!(
        "{:?} -- num remaining accounts: {:?}",
        boost_pda,
        remaining_accounts.len()
    );
    remaining_accounts
}

/// check if enough time has passed since last checkpoint
async fn check_for_time(
    client: &Client,
    checkpoint: &Checkpoint,
    boost_pda: &Pubkey,
) -> Result<()> {
    log::info!("{:?} -- checking if interval has elapsed", boost_pda);
    let clock = client.rpc.get_clock().await?;
    let time_since_last = clock.unix_timestamp - checkpoint.ts;
    if time_since_last < CHECKPOINT_INTERVAL {
        log::info!(
            "{:?} -- not enough time has passed since last checkpoint. Wait {} more seconds.",
            boost_pda,
            CHECKPOINT_INTERVAL - time_since_last
        );
        return Err(anyhow::anyhow!(ClockStillTicking));
    }
    log::info!("{:?} -- interval elapsed", boost_pda);
    Ok(())
}

async fn rebase_all(
    client: &Client,
    mint: &Pubkey,
    boost: &Pubkey,
    stake_accounts: &[Pubkey],
    lookup_tables: &[Pubkey],
) -> Result<()> {
    log::info!("{} -- rebasing stake accounts", boost);
    // pack instructions for rebase
    if stake_accounts.is_empty() {
        // if total stakers is zero
        // but the checkpoint interval is still passed,
        // use default account to reset checkpoint for new stakers
        let ix = ore_boost_api::sdk::rebase(client.keypair.pubkey(), *mint, Pubkey::default());
        log::info!(
            "{} -- remaining accounts is empty -- but checkpoint is still elpased. resetting.",
            boost
        );
        let sig = client.send_transaction(&[ix]).await?;
        log::info!("{} -- reset signature: {:?}", boost, sig);
    } else {
        // chunk stake accounts into batches
        let mut bundles: Vec<Vec<Instruction>> = vec![];
        for chunk in stake_accounts.chunks(MAX_ACCOUNTS_PER_TX) {
            // build transaction
            let mut transaction = vec![];
            for account in chunk {
                let signer = Arc::clone(&client.keypair);
                transaction.push(ore_boost_api::sdk::rebase(signer.pubkey(), *mint, *account));
            }
            bundles.push(transaction);
        }
        // bundle transactions
        for tx in bundles.chunks(5) {
            let bundle: Vec<&[Instruction]> = tx.iter().map(|vec| vec.as_slice()).collect();
            log::info!("{} -- submitting rebase", boost);
            let bundle_id = client
                .send_jito_bundle_with_luts(bundle.as_slice(), lookup_tables)
                .await?;
            log::info!("{} -- rebase bundle id: {:?}", boost, bundle_id);
        }
    }
    log::info!("{} -- checkpoint complete", boost);
    Ok(())
}
