use ore_utils::*;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{consts::*, state::*};

/// Errors if:
/// - Owner is not Boost program.
/// - Data is empty.
/// - Data cannot deserialize into a boost account.
/// - Expected to be writable, but is not.
pub fn load_boost(
    info: &AccountInfo<'_>,
    mint: &Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let boost_data = info.data.borrow();
    let boost = Boost::try_from_bytes(&boost_data)?;

    if boost.mint.ne(&mint) {
        return Err(ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not Boost program.
/// - Data is empty.
/// - Data cannot deserialize into a boost account.
/// - Expected to be writable, but is not.
pub fn load_any_boost(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    if info.data.borrow()[0].ne(&Boost::discriminator()) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not Boost program.
/// - Address does not match the expected address.
/// - Data is empty.
/// - Data cannot deserialize into a config account.
/// - Expected to be writable, but is not.
pub fn load_config(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.key.ne(&CONFIG_ADDRESS) {
        return Err(ProgramError::InvalidSeeds);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    if info.data.borrow()[0].ne(&Config::discriminator()) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not Boost program.
/// - Data is empty.
/// - Data cannot deserialize into a stake account.
/// - Stake authority does not match the expected address.
/// - Expected to be writable, but is not.
pub fn load_stake(
    info: &AccountInfo<'_>,
    authority: &Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let stake_data = info.data.borrow();
    let stake = Stake::try_from_bytes(&stake_data)?;

    if stake.authority.ne(&authority) {
        return Err(ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}
