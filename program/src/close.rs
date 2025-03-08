use ore_boost_api::state::Stake;
use steel::*;

/// Close closes a stake account.
pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, stake_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    stake_info
        .is_writable()?
        .as_account::<Stake>(&ore_boost_api::ID)?
        .assert_err(
            |p| p.authority == *signer_info.key,
            ProgramError::MissingRequiredSignature,
        )?
        .assert(|p| p.balance == 0)?
        .assert(|p| p.rewards == 0)?;
    system_program.is_program(&system_program::ID)?;

    // Return rent to signer.
    stake_info.close(signer_info)?;

    Ok(())
}
