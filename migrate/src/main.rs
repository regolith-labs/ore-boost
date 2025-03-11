mod client;
mod error;

use std::{sync::Arc, time::Duration};

use ore_boost_api::state::Boost;
use solana_sdk::{signature::Keypair, signer::Signer, stake};
use tokio::time::sleep;

use crate::client::{AsyncClient, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = client::Client::new()?;
    let client = std::sync::Arc::new(client);

    // Get pre migration numbers.
    let boosts_v1 = client.rpc.get_boosts_v1().await?;
    for boost_v1 in boosts_v1 {
        let boost_address = ore_boost_api::state::boost_pda(boost_v1.mint).0;
        let boost_v1_address = ore_boost_api_v1::state::boost_pda(boost_v1.mint).0;
        let boost = client.rpc.get_boost(&boost_address).await?;
        let mut stake_accounts = client
            .rpc
            .get_boost_v1_stake_accounts(&boost_v1_address)
            .await?;
        stake_accounts.sort_by_key(|(_, stake)| stake.id);
        for stake_v1 in stake_accounts {
            // let res = migrate_stake_account(client.clone(), boost_v1, stake_v1.1).await;
            let res = migrate_stake_account(client.clone(), boost, boost_v1, stake_v1.1).await;
            match res {
                Ok(_) => println!("Success"),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        // println!("Boost: {:?}", boost);
        // println!("Stake accounts: {:?}", stake_accounts);
    }
    // let boosts = client.get_boosts().await?;

    // Run migration.
    // TODO Fetch all v1 and v3 boosts.
    // TODO For each boost, fetch all stake accounts.
    // TODO For each stake account, submit a migrate instruction.

    // Verify migration.
    // TODO
    Ok(())
}

async fn migrate_stake_account(
    client: Arc<Client>,
    boost: Boost,
    boost_v1: ore_boost_api_v1::state::Boost,
    stake_v1: ore_boost_api_v1::state::Stake,
) -> anyhow::Result<()> {
    // Log pre migration state
    let boost_address = ore_boost_api::state::boost_pda(boost.mint).0;
    let stake_address = ore_boost_api::state::stake_pda(stake_v1.authority, boost_address).0;
    let boost_v1_address = ore_boost_api_v1::state::boost_pda(boost_v1.mint).0;
    let stake_v1_address =
        ore_boost_api_v1::state::stake_pda(stake_v1.authority, boost_v1_address).0;
    println!(
        "\nMigrating stake account ({}/{})",
        stake_v1.id + 1,
        boost_v1.total_stakers
    );
    println!("    Old address: {}", stake_v1_address);
    println!("    New address: {}", stake_address);
    println!("    Authority: {}", stake_v1.authority);
    println!("    Boost: {}", boost_v1.mint);
    println!("    Balance: {:?}", stake_v1.balance);
    println!("    Rewards: {:?}", stake_v1.rewards);

    // Submit migrate instruction
    let migrate_ix =
        ore_boost_api::sdk::migrate(client.keypair.pubkey(), stake_v1.authority, boost.mint);

    // Submit and confirm
    match client.send_transaction(&[migrate_ix]).await {
        Ok(sig) => println!("    OK: {}", sig),
        Err(e) => println!("    FAIL: {}", e),
    }

    sleep(Duration::from_secs(1)).await;

    // TODO Log post migration state (new and old stake accounts)
    let stake = client.rpc.get_stake(&stake_address).await?;
    let stake_v1 = client.rpc.get_stake_v1(&stake_v1_address).await?;
    println!("    Post balance (v1): {}", stake_v1.balance);
    println!("    Post rewards (v1): {}", stake_v1.rewards);
    println!("    Post balance: {}", stake.balance);
    println!("    Post rewards: {}", stake.rewards);

    Ok(())
}
