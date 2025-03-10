use ore_boost_api::state::{Boost, Stake};
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get pre migration numbers.

    // Run migration.
    // TODO Fetch all v1 and v3 boosts.
    // TODO For each boost, fetch all stake accounts.
    // TODO For each stake account, submit a migrate instruction.

    // Verify migration.
    // TODO

    println!("Hello, world!");
    Ok(())
}

fn migrate_stake_account(
    boost: Boost,
    boost_v1: ore_boost_api_v1::state::Boost,
    stake_v1: ore_boost_api_v1::state::Stake,
    signer: Keypair,
) -> anyhow::Result<()> {
    // Log pre migration state
    let stake_address = ore_boost_api::state::stake_pda(stake_v1.authority, boost.mint).0;
    let stake_v1_address = ore_boost_api_v1::state::stake_pda(stake_v1.authority, boost_v1.mint).0;
    println!(
        "Migrating stake account ({}/{})",
        stake_v1.id, boost_v1.total_stakers
    );
    println!("    Old address: {}", stake_v1_address);
    println!("    New address: {}", stake_address);
    println!("    Balance: {:?}", stake_v1.balance);
    println!("    Rewards: {:?}", stake_v1.rewards);

    // Submit migrate instruction
    let migrate_ix = ore_boost_api::sdk::migrate(signer.pubkey(), stake_v1.authority, boost.mint);

    // TODO Submit and confirm

    // TODO Log post migration state (new and old stake accounts)

    Ok(())
}
