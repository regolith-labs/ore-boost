use ore_boost_api::{
    instruction::Deposit,
    state::{Boost, Stake},
};
use steel::*;

/// Deposit adds tokens to a stake account to earn a multiplier.
pub fn process_deposit(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Deposit::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, boost_info, boost_tokens_info, mint_info, sender_info, stake_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info
        .to_account_mut::<Boost>(&ore_boost_api::ID)?
        .check_mut(|b| b.mint == *mint_info.key)?;
    boost_tokens_info
        .is_writable()?
        .to_associated_token_account(boost_info.key, mint_info.key)?;
    mint_info.to_mint()?;
    sender_info
        .is_writable()?
        .to_token_account()?
        .check(|t| t.owner == *signer_info.key)?
        .check(|t| t.mint == *mint_info.key)?;
    let stake = stake_info
        .to_account_mut::<Stake>(&ore_boost_api::ID)?
        .check_mut(|s| s.boost == *boost_info.key)?
        .check_mut(|s| s.authority == *signer_info.key)?;
    token_program.is_program(&spl_token::ID)?;

    // Update the stake balance.
    stake.balance = stake.balance.checked_add(amount).unwrap();

    // Update the boost balance.
    boost.total_stake = boost.total_stake.checked_add(amount).unwrap();

    // Update deposit timestamp.
    let clock = Clock::get().unwrap();
    stake.last_stake_at = clock.unix_timestamp;

    // Transfer tokens from signer to treasury
    transfer(
        signer_info,
        sender_info,
        boost_tokens_info,
        token_program,
        amount,
    )?;

    Ok(())
}
