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
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.balance == 0)?;
    system_program.is_program(&system_program::ID)?;

    // Close the stake account.
    stake_info.close(signer_info)?;

    Ok(())
}
