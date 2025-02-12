use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    str::FromStr,
};

use anyhow::Result;
use ore_boost_api::state::Stake;
use solana_sdk::{address_lookup_table, instruction::Instruction, pubkey::Pubkey, signer::Signer};

use crate::client::{AsyncClient, Client};

const MAX_ACCOUNTS_PER_LUT: usize = 256;

type LookupTables = Vec<Pubkey>;
/// sync lookup tables
///
/// add and/or extend lookup tables
/// for new stake accounts for next checkpoint
pub async fn sync(
    client: &Client,
    boost: &Pubkey,
    stake_accounts: &[(Pubkey, Stake)],
) -> Result<LookupTables> {
    log::info!("{} -- syncing lookup tables", boost);
    // read existing lookup table addresses
    let existing = read_file(boost)?;
    log::info!("{} -- existing lookup tables: {:?}", boost, existing);
    // fetch lookup table accounts for the stake addresses they hold
    let lookup_tables = client.rpc.get_lookup_tables(existing.as_slice()).await?;
    // filter for stake accounts that don't already have a lookup table
    let tabled_stake_account_addresses = lookup_tables
        .iter()
        .flat_map(|lut| lut.addresses.to_vec())
        .collect::<Vec<_>>();
    let untabled_stake_account_addresses = stake_accounts
        .iter()
        .filter_map(
            |(pubkey, _stake)| match tabled_stake_account_addresses.contains(pubkey) {
                true => None,
                false => Some(*pubkey),
            },
        )
        .collect::<Vec<_>>();
    log::info!(
        "{} -- num tabled addresses: {}",
        boost,
        tabled_stake_account_addresses.len()
    );
    log::info!(
        "{} -- num untabled addresses: {}",
        boost,
        untabled_stake_account_addresses.len()
    );
    if untabled_stake_account_addresses.is_empty() {
        return Ok(existing);
    }
    // check for a lookup table that still has capacity
    let capacity = lookup_tables
        .into_iter()
        .filter(|lut| lut.addresses.len().lt(&MAX_ACCOUNTS_PER_LUT))
        .collect::<Vec<_>>();
    let capacity = capacity.first();
    // if capacity, extend with new stake addresses
    let rest = match capacity {
        Some(capacity) => {
            log::info!("{} -- found lookup table with capacity", boost);
            // extend the lookup table that has capacity
            let space_remaining = MAX_ACCOUNTS_PER_LUT - capacity.addresses.len();
            let (extending, needs_allocation) =
                if space_remaining > untabled_stake_account_addresses.len() {
                    (untabled_stake_account_addresses, vec![])
                } else {
                    let (ext, na) = untabled_stake_account_addresses.split_at(space_remaining);
                    (ext.to_vec(), na.to_vec())
                };
            extend_lookup_table(client, boost, &capacity.key, extending.as_slice()).await?;
            // set aside the rest of the stake accounts for allocation in a new lookup table
            needs_allocation.to_vec()
        }
        None => untabled_stake_account_addresses,
    };
    // if remaining stake addresses, allocate new lookup table(s)
    if !rest.is_empty() {
        for chunk in rest.chunks(MAX_ACCOUNTS_PER_LUT) {
            // allocate new lookup table
            let lut_pda = create_lookup_table(client, boost).await?;
            write_file(&[lut_pda], boost)?;
            log::info!(
                "{} -- sleeping to allow lookup table creation to settle",
                boost
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            // extend this new lookup table
            extend_lookup_table(client, boost, &lut_pda, chunk).await?;
        }
    }
    // read latest lookup tables
    let lookup_tables = read_file(boost)?;
    Ok(lookup_tables)
}

async fn extend_lookup_table(
    client: &Client,
    boost: &Pubkey,
    lookup_table: &Pubkey,
    stake_accounts: &[Pubkey],
) -> Result<()> {
    log::info!("{} -- extending lookup table", boost);
    let mut bundles: Vec<Vec<Instruction>> = Vec::with_capacity(5);
    for chunk in stake_accounts.chunks(26) {
        let signer = client.keypair.pubkey();
        let extend_ix = address_lookup_table::instruction::extend_lookup_table(
            *lookup_table,
            signer,
            Some(signer),
            chunk.to_vec(),
        );
        bundles.push(vec![extend_ix]);
        if bundles.len().eq(&5) {
            let compiled: Vec<&[Instruction]> = bundles.iter().map(|vec| vec.as_slice()).collect();
            log::info!("{} -- sending extend instructions as bundle", boost);
            let bundle_id = client.send_jito_bundle(compiled.as_slice()).await?;
            log::info!("{} -- extend bundle id: {}", boost, bundle_id);
            bundles.clear();
        }
    }
    // submit last jito bundle
    if !bundles.is_empty() {
        log::info!("{} -- found left over extend bundles", boost);
        let compiled: Vec<&[Instruction]> = bundles.iter().map(|vec| vec.as_slice()).collect();
        log::info!("{} -- sending extend instructions as bundle", boost);
        let bundle_id = client.send_jito_bundle(compiled.as_slice()).await?;
        log::info!("{} -- extend bundle id: {}", boost, bundle_id);
    }
    Ok(())
}

async fn create_lookup_table(client: &Client, boost: &Pubkey) -> Result<Pubkey> {
    log::info!("{:?} -- opening new lookup table", boost);
    let clock = client.rpc.get_clock().await?;
    let signer = client.keypair.pubkey();
    // build and submit create instruction first
    let (create_ix, lut_pda) =
        address_lookup_table::instruction::create_lookup_table(signer, signer, clock.slot);
    let sig = client.send_transaction(&[create_ix]).await?;
    log::info!("{:?} -- new lookup table signature: {:?}", boost, sig);
    Ok(lut_pda)
}

fn write_file(luts: &[Lut], boost: &Pubkey) -> Result<()> {
    log::info!("{:?} -- writing new lookup tables", boost);
    let luts_path = luts_path()?;
    let path = format!("{}-{}", luts_path, boost);
    let mut file = OpenOptions::new()
        .create(true) // open or create
        .append(true) // append
        .open(path)?;
    for lut in luts {
        let line = format!("{}\n", lut);
        file.write_all(line.as_bytes())?;
    }
    log::info!("{:?} -- new lookup tables written", boost);
    Ok(())
}

type Lut = Pubkey;
fn read_file(boost: &Pubkey) -> Result<Vec<Lut>> {
    log::info!("{:?} -- reading prior lookup tables", boost);
    let luts_path = luts_path()?;
    let path = format!("{}-{}", luts_path, boost);
    // get or create file
    let file = match File::open(path.as_str()) {
        Ok(f) => f,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                log::info!(
                    "{} -- no prior lookup tables found, creating new cache file",
                    boost
                );
                let _file = File::create(path)?;
                log::info!("{} -- new cache file created", boost);
                return Ok(vec![]);
            }
            _ => {
                return Err(anyhow::anyhow!(err));
            }
        },
    };
    log::info!("{:?} -- found prior lookup tables file", boost);
    let mut luts = vec![];
    let reader = BufReader::new(file);
    // read lines
    for line_res in reader.lines() {
        // handle bytes
        let line = match line_res {
            Ok(line) => line,
            Err(err) => {
                log::error!("{} -- error decoding cache file {:?}", boost, err);
                continue;
            }
        };
        // decode
        match Pubkey::from_str(line.as_str()) {
            Ok(pubkey) => luts.push(pubkey),
            Err(err) => {
                log::error!("{} -- error decoding lut pubkey {:?}", boost, err);
                continue;
            }
        }
    }
    log::info!("{:?} -- parsed prior lookup tables", boost);
    Ok(luts)
}

fn luts_path() -> Result<String> {
    let path = std::env::var("LUTS_PATH")?;
    Ok(path)
}
