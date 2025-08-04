use ore_boost_api::{
    consts::{BOOST, RESERVE},
    instruction::Withdraw,
    state::{Boost, Config, Reserve, Stake},
};
use steel::*;

/// Withdraw unstakes tokens from a stake account.
pub fn process_withdraw(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Withdraw::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, beneficiary_info, boost_info, config_info, config_tokens_info, deposits_info, mint_info, reserve_info, reserve_tokens_info, stake_info, treasury_info, treasury_tokens_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    beneficiary_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint() == *mint_info.key)?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    let config = config_info.as_account_mut::<Config>(&ore_boost_api::ID)?;
    config_tokens_info
        .is_writable()?
        .as_associated_token_account(config_info.key, &ore_api::consts::MINT_ADDRESS)?;
    deposits_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, mint_info.key)?;
    mint_info.as_mint()?;
    reserve_info.as_account::<Reserve>(&ore_boost_api::ID)?;
    let reserve_tokens = reserve_tokens_info
        .is_writable()?
        .as_associated_token_account(reserve_info.key, &ore_api::consts::MINT_ADDRESS)?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    treasury_info.has_address(&ore_api::consts::TREASURY_ADDRESS)?;
    treasury_tokens_info.has_address(&ore_api::consts::TREASURY_TOKENS_ADDRESS)?;
    token_program.is_program(&spl_token::ID)?;

    // TODO Implement withdraw fee.

    // Withdraw stake.
    let amount = stake.withdraw(amount, boost, &clock, config, &reserve_tokens);

    // Transfer from reserve to config.
    transfer_signed(
        reserve_info,
        reserve_tokens_info,
        config_tokens_info,
        token_program,
        reserve_tokens.amount(),
        &[RESERVE],
    )?;

    // Withdraw deposits to beneficiary.
    transfer_signed(
        boost_info,
        deposits_info,
        beneficiary_info,
        token_program,
        amount,
        &[BOOST, mint_info.key.as_ref()],
    )?;

    Ok(())
}
