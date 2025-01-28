use ore_boost_api::prelude::*;
use steel::*;

/// Deposit adds tokens to a stake account.
pub fn process_deposit(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Deposit::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, boost_info, boost_deposits_info, mint_info, sender_info, stake_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info
        .as_account::<Boost>(&ore_boost_api::ID)?
        .assert(|b| b.mint == *mint_info.key)?;
    boost_deposits_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, mint_info.key)?;
    mint_info.as_mint()?;
    let sender = sender_info
        .is_writable()?
        .as_associated_token_account(signer_info.key, mint_info.key)?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    token_program.is_program(&spl_token::ID)?;

    // Normalize amount.
    let amount = amount.min(sender.amount);

    // Update balances.
    stake.balance_pending = stake.balance_pending.checked_add(amount).unwrap();

    // Update timestamps.
    let clock = Clock::get()?;
    stake.last_deposit_at = clock.unix_timestamp;

    // Transfer tokens.
    transfer(
        signer_info,
        sender_info,
        boost_deposits_info,
        token_program,
        amount,
    )?;

    Ok(())
}
