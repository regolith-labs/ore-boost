use ore_boost_api::state::{Boost, Stake};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};
use steel::*;

/// Closes a stake account.
pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer, boost_info, stake_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer.is_signer()?;
    boost_info.to_account::<Boost>(&ore_boost_api::ID)?;
    stake_info
        .is_writable()?
        .to_account::<Stake>(&ore_boost_api::ID)?
        .check(|s| s.balance > 0)?;
    system_program.is_program(&system_program::ID)?;

    // Realloc data to zero.
    stake_info.realloc(0, true)?;

    // Send remaining lamports to signer.
    **signer.lamports.borrow_mut() += stake_info.lamports();
    **stake_info.lamports.borrow_mut() = 0;

    Ok(())
}
