mod client;
mod error;

use std::{sync::Arc, time::Duration};

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

    let boosts = client.rpc.get_boosts_v3().await?;
    for boost in boosts {
        let boost_v3_address = ore_boost_api_v3::state::boost_pda(boost.mint).0;
        let stake_accounts = client
            .rpc
            .get_boost_v3_stake_accounts(&boost_v3_address)
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
