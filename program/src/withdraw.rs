use ore_boost_api::{
    consts::BOOST,
    error::BoostError,
    instruction::Withdraw,
    state::{Boost, Stake},
};
use steel::*;

/// Withdraw unstakes tokens from a stake account.
pub fn process_withdraw(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // TODO

    // Parse args.
    // let args = Withdraw::try_from_bytes(data)?;
    // let amount = u64::from_le_bytes(args.amount);

    // // Load accounts.
    // let [signer_info, beneficiary_info, boost_info, boost_deposits_info, mint_info, stake_info, token_program] =
    //     accounts
    // else {
    //     return Err(ProgramError::NotEnoughAccountKeys);
    // };
    // signer_info.is_signer()?;
    // beneficiary_info
    //     .is_writable()?
    //     .as_token_account()?
    //     .assert(|t| t.mint == *mint_info.key)?;
    // let boost = boost_info
    //     .as_account_mut::<Boost>(&ore_boost_api::ID)?
    //     .assert_mut(|b| b.mint == *mint_info.key)?
    //     .assert_mut_err(|b| b.locked == 0, BoostError::BoostLocked.into())?;
    // boost_deposits_info
    //     .is_writable()?
    //     .as_associated_token_account(boost_info.key, mint_info.key)?;
    // mint_info.as_mint()?;
    // let stake = stake_info
    //     .as_account_mut::<Stake>(&ore_boost_api::ID)?
    //     .assert_mut(|s| s.authority == *signer_info.key)?
    //     .assert_mut(|s| s.boost == *boost_info.key)?
    //     .assert_mut(|s| s.balance_pending.checked_add(s.balance).unwrap() >= amount)?;
    // token_program.is_program(&spl_token::ID)?;

    // // Calculate how much to withdraw from pending vs committed stake
    // let pending_withdraw = amount.min(stake.balance_pending);
    // let committed_withdraw = amount.checked_sub(pending_withdraw).unwrap();

    // // Update the pending stake balance
    // stake.balance_pending = stake.balance_pending.checked_sub(pending_withdraw).unwrap();

    // // Update the committed stake balance
    // stake.balance = stake.balance.checked_sub(committed_withdraw).unwrap();

    // // Update the boost balance
    // boost.total_deposits = boost.total_deposits.checked_sub(committed_withdraw).unwrap();

    // // Transfer tokens from boost to beneficiary
    // transfer_signed(
    //     boost_info,
    //     boost_deposits_info,
    //     beneficiary_info,
    //     token_program,
    //     amount,
    //     &[BOOST, mint_info.key.as_ref()],
    // )?;

    Ok(())
}
