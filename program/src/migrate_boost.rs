use ore_api::state::Proof;
use ore_boost_api::{consts::BOOST, state::*};
use solana_program::log::sol_log;
use steel::*;
use sysvar::rent::Rent;

pub fn process_migrate_boost(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, boost_proof_info, boost_rewards_info, config_info, ore_mint_info, rewards_info, treasury_info, treasury_tokens_info, system_program, token_program, ore_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    sol_log("A1");

    signer_info.is_signer()?;
    sol_log("A2");
    let (expires_at, mint, mut rewards_factor, total_deposits, total_stakers, withdraw_fee) = {
        let boost = boost_info.as_account::<OldBoost>(&ore_boost_api::ID)?;
        (
            boost.expires_at,
            boost.mint,
            boost.rewards_factor,
            boost.total_deposits,
            boost.total_stakers,
            boost.withdraw_fee,
        )
    };
    sol_log("A3");

    let boost_proof = boost_proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *boost_info.key)?;

    sol_log("A4");

    let boost_rewards =
        boost_rewards_info.as_associated_token_account(boost_info.key, ore_mint_info.key)?;

    sol_log("A5");

    let config = config_info
        .as_account_mut::<Config>(&ore_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;

    sol_log("A6");

    rewards_info.as_associated_token_account(config_info.key, ore_mint_info.key)?;

    sol_log("A7");

    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;

    sol_log("A8");

    sol_log("A");

    if boost_proof.balance > 0 {
        rewards_factor += Numeric::from_fraction(boost_proof.balance, total_deposits);
        invoke_signed(
            &ore_api::sdk::claim(*boost_info.key, *rewards_info.key, boost_proof.balance),
            &[
                boost_info.clone(),
                rewards_info.clone(),
                boost_proof_info.clone(),
                treasury_info.clone(),
                treasury_tokens_info.clone(),
                token_program.clone(),
                ore_program.clone(),
            ],
            &ore_boost_api::ID,
            &[BOOST, mint.as_ref()],
        )?;
    }

    // Record old values
    let rewards = rewards_info.as_associated_token_account(config_info.key, ore_mint_info.key)?;
    let pre_transfer_rewards_amount = rewards.amount();
    let rewards_to_transfer = if mint == ore_api::consts::MINT_ADDRESS {
        boost_rewards.amount() - total_deposits
    } else {
        boost_rewards.amount()
    };

    sol_log("B");

    // Realloc old boost account
    // let rent = Rent::get()?;
    let new_size = 8 + std::mem::size_of::<Boost>();
    boost_info.realloc(new_size, false)?;
    // let required_lamports = rent.minimum_balance(new_size);
    // let current_lamports = boost_info.lamports();
    // boost_info.send(current_lamports - required_lamports, signer_info);

    sol_log("C");

    // Initialize the new boost account.
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    boost.expires_at = expires_at;
    boost.mint = mint;
    boost.weight = 0;
    boost.rewards_factor = rewards_factor;
    boost.total_deposits = total_deposits;
    boost.total_stakers = total_stakers;
    boost.withdraw_fee = withdraw_fee;

    sol_log("D");

    // Transfer all staking rewards to the new global rewards account
    transfer_signed(
        boost_info,
        boost_rewards_info,
        rewards_info,
        token_program,
        rewards_to_transfer,
        &[BOOST, mint.as_ref()],
    )?;

    sol_log("E");

    // Assert migration was successful
    assert_eq!(boost.expires_at, expires_at);
    assert_eq!(boost.mint, mint);
    assert_eq!(boost.weight, 0);
    assert_eq!(boost.rewards_factor, rewards_factor);
    assert_eq!(boost.total_deposits, total_deposits);
    assert_eq!(boost.total_stakers, total_stakers);
    assert_eq!(boost.withdraw_fee, withdraw_fee);
    assert_eq!(boost_proof.balance, 0);

    sol_log("F");

    // Assert rewards were transferred correctly
    let rewards = rewards_info.as_associated_token_account(config_info.key, ore_mint_info.key)?;
    let boost_rewards =
        boost_rewards_info.as_associated_token_account(boost_info.key, ore_mint_info.key)?;
    assert_eq!(
        rewards.amount(),
        pre_transfer_rewards_amount + rewards_to_transfer
    );
    if boost.mint == ore_api::consts::MINT_ADDRESS {
        assert_eq!(boost_rewards.amount(), boost.total_deposits); // This account doubled as a vault for deposits and rewards
    } else {
        assert_eq!(boost_rewards.amount(), 0);
    }

    Ok(())
}
