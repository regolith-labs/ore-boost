mod client;
mod error;

use std::{str::FromStr, sync::Arc, time::Duration};

use ore_boost_api::state::Boost;
use solana_sdk::signer::Signer;
use steel::{Numeric, Pubkey};
use tokio::time::sleep;

use crate::client::{AsyncClient, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = client::Client::new()?;
    let client = std::sync::Arc::new(client);

    // Get pre migration numbers.
    let boosts_v1 = client.rpc.get_boosts_v1().await?;
    for boost_v1 in boosts_v1 {
        // Only do ORE-HNT
        // if boost_v1
        //     .mint
        //     .ne(&Pubkey::from_str("oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp").unwrap())
        // {
        //     println!("Skipping boost: {}", boost_v1.mint);
        //     continue;
        // }

        let boost_address = ore_boost_api::state::boost_pda(boost_v1.mint).0;
        let boost_v1_address = ore_boost_api_v1::state::boost_pda(boost_v1.mint).0;
        let boost = client.rpc.get_boost(&boost_address).await?;

        println!("Boost: {:?}", boost);
        // println!("Boost v1: {:?}", boost_v1);

        // Migrate
        // let mut stake_accounts = client
        //     .rpc
        //     .get_boost_v1_stake_accounts(&boost_v1_address)
        //     .await?;
        // stake_accounts.sort_by_key(|(_, stake)| stake.id);
        // for stake_v1 in stake_accounts {
        //     // if stake_v1.1.id < boost.total_stakers {
        //     //     println!("Skipping...");
        //     //     continue;
        //     // }
        //     let res = migrate_stake_account(client.clone(), boost, boost_v1, stake_v1.1).await;
        //     match res {
        //         Ok(_) => println!("Success"),
        //         Err(e) => println!("Error: {:?}", e),
        //     }
        // }

        verify_rewards_factor_stake(client.clone(), boost_address).await?;

        // Refresh stake balances. Check they are nulled.
        sleep(Duration::from_secs(5)).await;
        let mut stake_accounts = client
            .rpc
            .get_boost_v1_stake_accounts(&boost_v1_address)
            .await?;
        stake_accounts.sort_by_key(|(_, stake)| stake.id);
        for stake_v1 in stake_accounts {
            check_balances(boost_v1, stake_v1.1).await?;
        }

        // Assert boost
        sleep(Duration::from_secs(5)).await;
        let boost = client.rpc.get_boost(&boost_address).await?;
        assert_eq!(boost.total_stakers, boost_v1.total_stakers);
        println!("Boost: {:?}", boost);
        println!("Boost v1: {:?}", boost_v1);

        // panic!("Migrated 1 boost. Stopping.");
    }
    Ok(())
}

async fn check_balances(
    boost_v1: ore_boost_api_v1::state::Boost,
    stake_v1: ore_boost_api_v1::state::Stake,
) -> anyhow::Result<()> {
    println!(
        "Stake v1: ({}/{}) {} {}",
        stake_v1.id, boost_v1.total_stakers, stake_v1.balance, stake_v1.rewards
    );
    assert!(stake_v1.balance == 0);
    assert!(stake_v1.rewards == 0);
    Ok(())
}

async fn verify_rewards_factor(client: Arc<Client>, boost_address: Pubkey) -> anyhow::Result<()> {
    let mut net_deposits = 0;
    let mut net_rewards = 0;
    let stake_accounts = client.rpc.get_boost_stake_accounts(&boost_address).await?;
    for stake in stake_accounts {
        net_deposits += stake.1.balance;
        net_rewards += stake.1.rewards;
    }
    println!("Net deposits: {}", net_deposits);
    println!("Net rewards: {}", net_rewards);

    let expected_rewards_factor = Numeric::from_fraction(net_rewards, net_deposits);
    let boost = client.rpc.get_boost(&boost_address).await?;
    assert_eq!(boost.rewards_factor, expected_rewards_factor);
    Ok(())
}

async fn verify_rewards_factor_stake(
    client: Arc<Client>,
    boost_address: Pubkey,
) -> anyhow::Result<()> {
    let boost = client.rpc.get_boost(&boost_address).await?;

    let mut net_deposits = 0;
    let mut net_rewards = 0;
    let stake_accounts = client.rpc.get_boost_stake_accounts(&boost_address).await?;
    for stake in stake_accounts {
        println!(
            "Stake: {:?} {} {} ",
            stake.1.last_rewards_factor, stake.1.balance, stake.1.rewards,
        );
        net_deposits += stake.1.balance;
        net_rewards += stake.1.rewards;
        assert!(stake.1.last_rewards_factor == boost.rewards_factor);
    }
    println!("Net deposits: {}", net_deposits);
    println!("Net rewards: {}", net_rewards);

    let expected_rewards_factor = Numeric::from_fraction(net_rewards, net_deposits);
    assert_eq!(boost.rewards_factor, expected_rewards_factor);

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

    // Final pass check
    let stake = client.rpc.get_stake(&stake_address).await?;
    if stake.last_rewards_factor != Numeric::ZERO {
        return Ok(());
    }

    // Submit migrate instruction
    let migrate_ix =
        ore_boost_api::sdk::migrate(client.keypair.pubkey(), stake_v1.authority, boost.mint);

    // Submit and confirm
    match client.send_transaction(&[migrate_ix]).await {
        Ok(sig) => println!("    OK: https://solscan.io/tx/{}", sig),
        Err(e) => println!("    FAIL: {}", e),
    }

    // TODO Log post migration state (new and old stake accounts)
    sleep(Duration::from_secs(1)).await;
    // let stake = client.rpc.get_stake(&stake_address).await?;
    // let stake_v1 = client.rpc.get_stake_v1(&stake_v1_address).await?;
    // println!("    Post balance (v1): {}", stake_v1.balance);
    // println!("    Post rewards (v1): {}", stake_v1.rewards);
    // println!("    Post balance: {}", stake.balance);
    // println!("    Post rewards: {}", stake.rewards);

    Ok(())
}
