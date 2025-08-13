use ore_boost_api::consts::{CONFIG, RESERVE};
use ore_boost_api::instruction::Claim;
use ore_boost_api::state::{Boost, Config, Reserve, Stake};
use steel::*;

/// Claim distributes rewards to a staker.
pub fn process_claim(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    panic!("Program is in migration mode");

    // Parse args.
    let args = Claim::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts
    let clock = Clock::get()?;
    let [signer_info, beneficiary_info, boost_info, config_info, config_tokens_info, reserve_info, reserve_tokens_info, stake_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    beneficiary_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint() == ore_api::consts::MINT_ADDRESS)?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    let config = config_info.as_account_mut::<Config>(&ore_boost_api::ID)?;
    config_tokens_info
        .is_writable()?
        .as_associated_token_account(config_info.key, &ore_api::consts::MINT_ADDRESS)?;
    reserve_info.as_account::<Reserve>(&ore_boost_api::ID)?;
    let reserve_tokens = reserve_tokens_info
        .is_writable()?
        .as_associated_token_account(reserve_info.key, &ore_api::consts::MINT_ADDRESS)?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    token_program.is_program(&spl_token::ID)?;

    // Claim rewards.
    let amount = stake.claim(amount, boost, &clock, config, &reserve_tokens);

    // Transfer from reserve to config.
    transfer_signed(
        reserve_info,
        reserve_tokens_info,
        config_tokens_info,
        token_program,
        reserve_tokens.amount(),
        &[RESERVE],
    )?;

    // Transfer tokens to beneficiary.
    transfer_signed(
        config_info,
        config_tokens_info,
        beneficiary_info,
        token_program,
        amount,
        &[CONFIG],
    )?;

    Ok(())
}
