mod client;
mod error;

use std::{sync::Arc, time::Duration, u64};

use ore_boost_api::state::Boost;
use solana_sdk::signer::Signer;
use spl_token::amount_to_ui_amount;
use steel::Numeric;
use tokio::time::sleep;
use url::form_urlencoded::parse;

use crate::client::{AsyncClient, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = client::Client::new()?;
    let client = std::sync::Arc::new(client);

    let old_config = client.rpc.get_config_old().await?;
    println!("Old config: {:?}", old_config);

    migrate_config(&client).await?;

    let config = client.rpc.get_config().await?;
    println!("Config: {:?}", config);

    Ok(())
}

async fn migrate_config(client: &Client) -> anyhow::Result<()> {
    let ix = ore_boost_api::sdk::migrate_config(client.keypair.pubkey());
    match client.send_transaction(&[ix]).await {
        Ok(sig) => println!("    OK: {}", sig),
        Err(e) => println!("    FAIL: {}", e),
    }
    Ok(())
}

async fn get_addresses(client: &Client) -> anyhow::Result<()> {
    // Print program addresses
    let config_address = ore_boost_api::state::config_pda().0;
    let boosts = client.rpc.get_boosts_old().await?;
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

async fn validate_reserves(client: &Client) -> anyhow::Result<()> {
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
