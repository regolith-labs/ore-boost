use ore_api::state::Proof;
use ore_boost_api::{consts::BOOST, state::*};
use steel::*;

pub fn process_migrate_boost(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, boost_proof_info, boost_rewards_info, config_info, ore_mint_info, rewards_info, system_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<OldBoost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.mint == *signer_info.key)?;
    boost_proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *boost_info.key)?
        .assert(|p| p.balance == 0)?;
    let boost_rewards =
        boost_rewards_info.as_associated_token_account(boost_info.key, ore_mint_info.key)?;
    config_info
        .as_account::<Config>(&ore_boost_api::ID)?
        .assert(|c| c.admin == *signer_info.key)?;
    let rewards = rewards_info.as_associated_token_account(config_info.key, ore_mint_info.key)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Record old values
    let expires_at = boost.expires_at;
    let mint = boost.mint;
    let rewards_factor = boost.rewards_factor;
    let total_deposits = boost.total_deposits;
    let total_stakers = boost.total_stakers;
    let withdraw_fee = boost.withdraw_fee;
    let pre_transfer_rewards_amount = rewards.amount();
    let rewards_to_transfer = if boost.mint == ore_api::consts::MINT_ADDRESS {
        boost_rewards.amount() - boost.total_deposits
    } else {
        boost_rewards.amount()
    };

    // Close the old boost account
    boost_info.close(signer_info)?;

    // Initialize the new boost account.
    create_program_account::<Boost>(
        boost_info,
        system_program,
        signer_info,
        &ore_boost_api::ID,
        &[BOOST, mint.as_ref()],
    )?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    boost.expires_at = expires_at;
    boost.mint = mint;
    boost.weight = 0;
    boost.rewards_factor = rewards_factor;
    boost.total_deposits = total_deposits;
    boost.total_stakers = total_stakers;
    boost.withdraw_fee = withdraw_fee;

    // Transfer all staking rewards to the new global rewards account
    transfer_signed(
        boost_info,
        boost_rewards_info,
        rewards_info,
        token_program,
        rewards_to_transfer,
        &[BOOST, mint.as_ref()],
    )?;

    // Assert migration was successful
    assert_eq!(boost.expires_at, expires_at);
    assert_eq!(boost.mint, mint);
    assert_eq!(boost.weight, 0);
    assert_eq!(boost.rewards_factor, rewards_factor);
    assert_eq!(boost.total_deposits, total_deposits);
    assert_eq!(boost.total_stakers, total_stakers);
    assert_eq!(boost.withdraw_fee, withdraw_fee);
    boost_proof_info.is_empty()?;

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
