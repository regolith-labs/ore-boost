mod client;
mod error;

use std::{sync::Arc, time::Duration, u64};

use ore_boost_api::state::Boost;
use solana_program::pubkey;
use solana_sdk::signer::Signer;
use spl_token::amount_to_ui_amount;
use steel::{Numeric, Pubkey};
use tokio::time::sleep;
use url::form_urlencoded::parse;

use crate::client::{AsyncClient, Client};

// Load boost addresses
const BOOST_MINT_ADDRESSES: [Pubkey; 6] = [
    pubkey!("7G3dfZkSk1HpDGnyL37LMBbPEgT4Ca6vZmZPUyi2syWt"),
    pubkey!("8H8rPiWW4iTFCfEkSnf7jpqeNpFfvdH9gLouAL3Fe2Zx"),
    pubkey!("DrSS5RM7zUd9qjUEdDaf31vnDUSbCrMto6mjqTrHFifN"),
    pubkey!("9BAWwtAZiF4XJC6vArPM8JhtgKXfeoeo9FJHeR3PEGac"),
    pubkey!("meUwDp23AaxhiNKaQCyJ2EAF2T4oe1gSkEkGXSRVdZb"),
    pubkey!("oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp"),
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = client::Client::new()?;
    let client = std::sync::Arc::new(client);

    // Get addresses
    // get_addresses(&client).await?;

    // Get stake addresses
    // get_stake_addresses(&client).await?;

    // Migrate config
    // migrate_config(&client).await?;

    // Migrate boosts
    // migrate_boosts(&client).await?;

    // Update weights
    // update_weights(&client).await?;

    // Validate reserves
    validate_reserves(&client).await?;

    Ok(())
}

async fn update_weights(client: &Client) -> anyhow::Result<()> {
    // Build update weights instructions
    let mut ixs = vec![];
    for mint_address in BOOST_MINT_ADDRESSES {
        let boost_address = ore_boost_api::state::boost_pda(mint_address).0;
        let boost = client.rpc.get_boost(&boost_address).await?;
        let weight = match boost.mint.to_string().as_str() {
            "7G3dfZkSk1HpDGnyL37LMBbPEgT4Ca6vZmZPUyi2syWt" => 0,
            "8H8rPiWW4iTFCfEkSnf7jpqeNpFfvdH9gLouAL3Fe2Zx" => 5000,
            "DrSS5RM7zUd9qjUEdDaf31vnDUSbCrMto6mjqTrHFifN" => 4000,
            "9BAWwtAZiF4XJC6vArPM8JhtgKXfeoeo9FJHeR3PEGac" => 4000,
            "meUwDp23AaxhiNKaQCyJ2EAF2T4oe1gSkEkGXSRVdZb" => 0,
            "oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp" => 500,
            _ => 0,
        };
        let ix = ore_boost_api::sdk::update_boost(
            client.keypair.pubkey(),
            boost_address,
            boost.expires_at,
            weight,
        );
        ixs.push(ix);
    }

    // Send migrate boosts instructions
    match client.send_transaction(&ixs).await {
        Ok(sig) => println!("    OK: {}", sig),
        Err(e) => println!("    FAIL: {}", e),
    }
    sleep(Duration::from_secs(10)).await;

    Ok(())
}

async fn migrate_boosts(client: &Client) -> anyhow::Result<()> {
    // Build migrate boosts instructions
    let mut ixs = vec![];
    for mint_address in BOOST_MINT_ADDRESSES {
        let ix = ore_boost_api::sdk::migrate_boost(client.keypair.pubkey(), mint_address);
        ixs.push(ix);
    }

    // Send migrate boosts instructions
    match client.send_transaction(&ixs).await {
        Ok(sig) => println!("    OK: {}", sig),
        Err(e) => println!("    FAIL: {}", e),
    }
    sleep(Duration::from_secs(10)).await;

    // Lookup new boosts
    for mint_address in BOOST_MINT_ADDRESSES {
        let boost_address = ore_boost_api::state::boost_pda(mint_address).0;
        let boost = client.rpc.get_boost(&boost_address).await?;
        println!("Boost: {:?}", boost);
    }

    Ok(())
}

async fn migrate_config(client: &Client) -> anyhow::Result<()> {
    // Lookup old config
    let old_config = client.rpc.get_config_old().await?;
    println!("Old config: {:?}", old_config);

    // Migrate config
    let ix = ore_boost_api::sdk::migrate_config(client.keypair.pubkey());
    match client.send_transaction(&[ix]).await {
        Ok(sig) => println!("    OK: {}", sig),
        Err(e) => println!("    FAIL: {}", e),
    }
    sleep(Duration::from_secs(10)).await;

    // Lookup new config
    let config = client.rpc.get_config().await?;
    println!("Config: {:?}", config);

    Ok(())
}

async fn get_addresses(client: &Client) -> anyhow::Result<()> {
    // Print program addresses
    let config_address = ore_boost_api::state::config_pda().0;
    let boosts = client.rpc.get_boosts_old().await?;
    println!("Treasury: {:?}", ore_api::consts::TREASURY_ADDRESS);
    println!(
        "Treasury tokens: {:?}",
        ore_api::consts::TREASURY_TOKENS_ADDRESS
    );
    println!("Config: {:?}", config_address);
    println!("Boost program: {:?}", ore_boost_api::ID);
    println!("Ore program: {:?}", ore_api::ID);

    // Print boost addresses
    for boost in boosts {
        let boost_address = ore_boost_api::state::boost_pda(boost.mint).0;
        let boost_rewards_address = spl_associated_token_account::get_associated_token_address(
            &boost_address,
            &ore_api::consts::MINT_ADDRESS,
        );
        let boost_proof_address = ore_api::state::proof_pda(boost_address).0;
        println!("Boost mint: {}", boost.mint);
        println!("Boost address: {}", boost_address);
        println!("Boost rewards address: {}", boost_rewards_address);
        println!("Boost proof address: {}", boost_proof_address);
    }

    Ok(())
}

async fn get_stake_addresses(client: &Client) -> anyhow::Result<()> {
    for (i, mint_address) in BOOST_MINT_ADDRESSES.iter().enumerate() {
        let boost_address = ore_boost_api::state::boost_pda(*mint_address).0;
        let stake_accounts = client.rpc.get_boost_stake_accounts(&boost_address).await?;
        for (stake_address, _stake) in stake_accounts {
            println!("Stake address: {}", stake_address);
        }
    }
    Ok(())
}

async fn validate_reserves(client: &Client) -> anyhow::Result<()> {
    let mut rewards = 0;
    let config_address = ore_boost_api::state::config_pda().0;
    let config = client.rpc.get_config().await?;
    let proof_address = ore_api::state::proof_pda(config_address).0;
    let proof = client.rpc.get_proof(&proof_address).await?;
    let new_rewards_factor =
        config.rewards_factor + Numeric::from_fraction(proof.balance, config.total_weight);

    // Calculate debts
    let boosts = client.rpc.get_boosts().await?;
    for boost in boosts {
        // Accumulate boost rewards factor
        let boost_address = ore_boost_api::state::boost_pda(boost.mint).0;
        let stake_accounts = client.rpc.get_boost_stake_accounts(&boost_address).await?;
        let mut boost_rewards_factor = boost.rewards_factor;
        if boost.rewards_factor < new_rewards_factor {
            let accumulated_rewards = new_rewards_factor - boost.rewards_factor;
            if accumulated_rewards < Numeric::ZERO {
                panic!("accumulated_rewards < 0");
            }
            let weighted_rewards = accumulated_rewards * Numeric::from_u64(boost.weight);
            boost_rewards_factor += weighted_rewards / Numeric::from_u64(boost.total_deposits);
        }

        // Accumulate debts to stakers
        for (_stake_address, stake) in stake_accounts {
            rewards += stake.rewards;
            if stake.last_rewards_factor < boost_rewards_factor {
                let accumualted_rewards = boost_rewards_factor - stake.last_rewards_factor;
                if accumualted_rewards < Numeric::ZERO {
                    panic!("accumualted_rewards < 0");
                }
                let personal_rewards = accumualted_rewards * Numeric::from_u64(stake.balance);
                rewards += personal_rewards.to_u64();
            }
        }
    }

    // Calculate reserves
    let rewards_ata = spl_associated_token_account::get_associated_token_address(
        &config_address,
        &ore_api::consts::MINT_ADDRESS,
    );
    let token_account = client.rpc.get_token_account_balance(&rewards_ata).await?;
    let reserves = proof.balance + token_account.amount.parse::<u64>().unwrap();

    // Check reserves
    println!(
        "Rewards: {}",
        amount_to_ui_amount(rewards, ore_api::consts::TOKEN_DECIMALS)
    );
    println!(
        "Reserves: {}",
        amount_to_ui_amount(reserves, ore_api::consts::TOKEN_DECIMALS)
    );
    println!(
        "Proof: {}",
        amount_to_ui_amount(proof.balance, ore_api::consts::TOKEN_DECIMALS)
    );
    if reserves < rewards {
        println!("Insufficient reserves!");
        println!(
            "Difference: {}\n",
            amount_to_ui_amount(rewards - reserves, ore_api::consts::TOKEN_DECIMALS)
        );
    } else {
        println!("OK\n");
    }

    Ok(())
}

async fn validate_reserves_old(client: &Client) -> anyhow::Result<()> {
    let boosts = client.rpc.get_boosts_old().await?;
    for boost in boosts {
        let boost_v3_address = ore_boost_api::state::boost_pda(boost.mint).0;
        let stake_accounts = client
            .rpc
            .get_boost_stake_accounts(&boost_v3_address)
            .await?;

        let rewards_ata = spl_associated_token_account::get_associated_token_address(
            &boost_v3_address,
            &ore_api::consts::MINT_ADDRESS,
        );

        let token_account = client.rpc.get_token_account_balance(&rewards_ata).await?;
        let proof_address = ore_api::state::proof_pda(boost_v3_address).0;
        let proof = client.rpc.get_proof(&proof_address).await?;

        let new_boost_rewards_factor =
            boost.rewards_factor + Numeric::from_fraction(proof.balance, boost.total_deposits);

        let mut rewards = 0;
        for (_stake_address, stake) in stake_accounts {
            rewards += stake.rewards;
            if stake.last_rewards_factor < new_boost_rewards_factor {
                let accumualted_rewards = new_boost_rewards_factor - stake.last_rewards_factor;
                if accumualted_rewards < Numeric::ZERO {
                    panic!("accumualted_rewards < 0");
                }
                let personal_rewards = accumualted_rewards * Numeric::from_u64(stake.balance);
                rewards += personal_rewards.to_u64();
            }
        }

        // Check rewards
        let mut token_amount = token_account.amount.parse::<u64>().unwrap();
        if boost.mint == ore_api::consts::MINT_ADDRESS {
            token_amount -= boost.total_deposits;
        }
        let reserves = proof.balance + token_amount;
        println!("Boost: {}", boost.mint);
        println!(
            "Rewards: {}",
            amount_to_ui_amount(rewards, ore_api::consts::TOKEN_DECIMALS)
        );
        println!(
            "Reserves: {}",
            amount_to_ui_amount(reserves, ore_api::consts::TOKEN_DECIMALS)
        );
        println!(
            "Proof: {}",
            amount_to_ui_amount(proof.balance, ore_api::consts::TOKEN_DECIMALS)
        );
        if reserves < rewards {
            println!("Insufficient reserves!");
            println!(
                "Difference: {}\n",
                amount_to_ui_amount(rewards - reserves, ore_api::consts::TOKEN_DECIMALS)
            );
        } else {
            println!("OK\n");
        }
    }

    Ok(())
}
