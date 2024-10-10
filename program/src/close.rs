use ore_boost_api::prelude::*;
use steel::*;

/// Closes a stake account.
pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, stake_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    stake_info
        .to_account_mut::<Stake>(&ore_boost_api::ID)?
        .check_mut(|s| s.authority == *signer_info.key)?
        .check_mut(|s| s.balance == 0)?;
    system_program.is_program(&system_program::ID)?;

    // Realloc data to zero.
    stake_info.realloc(0, true)?;

    // Send remaining lamports to signer.
    **signer_info.lamports.borrow_mut() += stake_info.lamports();
    **stake_info.lamports.borrow_mut() = 0;

    Ok(())
}
