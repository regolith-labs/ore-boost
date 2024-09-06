use ore_boost_api::{
    loaders::{load_any_boost, load_stake},
    state::Stake,
};
use ore_utils::*;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};

/// Closes a stake account.
pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer, boost_info, stake_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_any_boost(boost_info, false)?;
    load_stake(stake_info, signer.key, boost_info.key, true)?;
    load_program(system_program, system_program::id())?;

    // Validate stake balance is zero.
    let stake_data = stake_info.data.borrow();
    let stake = Stake::try_from_bytes(&stake_data)?;
    if stake.balance.gt(&0) {
        return Err(ProgramError::InvalidAccountData);
    }
    drop(stake_data);

    // Realloc data to zero.
    stake_info.realloc(0, true)?;

    // Send remaining lamports to signer.
    **signer.lamports.borrow_mut() += stake_info.lamports();
    **stake_info.lamports.borrow_mut() = 0;

    Ok(())
}
