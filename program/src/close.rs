use ore_boost_api::prelude::*;
use steel::*;

/// Close closes a stake account.
pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, stake_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    stake_info
        .is_writable()?
        .as_account::<Stake>(&ore_boost_api::ID)?
        .assert_err(
            |p| p.authority == *signer_info.key,
            ProgramError::MissingRequiredSignature,
        )?
        .assert(|p| p.boost == *boost_info.key)?
        .assert(|p| p.balance == 0)?
        .assert(|p| p.rewards == 0)?;
    system_program.is_program(&system_program::ID)?;

    // Update boost total stakers
    boost.total_stakers -= 1;

    // Return rent to signer.
    stake_info.close(signer_info)?;

    Ok(())
}
