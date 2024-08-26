use std::mem::size_of;

use ore_boost_api::{
    consts::STAKE,
    instruction::Open,
    state::{Boost, Stake},
};
use ore_utils::*;
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult,
    program_error::ProgramError, system_program, sysvar::Sysvar,
};

/// Open ...
pub fn process_open(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Open::try_from_bytes(data)?;

    // Load accounts.
    let [signer, boost_info, mint_info, stake_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_uninitialized_pda(
        boost_info,
        &[STAKE, boost_info.key.as_ref(), signer.key.as_ref()],
        args.stake_bump,
        &ore_boost_api::id(),
    )?;
    load_any_mint(mint_info, false)?;
    load_program(system_program, system_program::id())?;

    // Get clock
    let clock = Clock::get().unwrap();

    // Initialize the boost account.
    create_pda(
        stake_info,
        &ore_boost_api::id(),
        8 + size_of::<Boost>(),
        &[STAKE, boost_info.key.as_ref(), signer.key.as_ref()],
        system_program,
        signer,
    )?;
    let mut stake_data = stake_info.data.borrow_mut();
    stake_data[0] = Stake::discriminator();
    let stake = Stake::try_from_bytes_mut(&mut stake_data)?;
    stake.authority = *signer.key;
    stake.balance = 0;
    stake.last_stake_at = clock.unix_timestamp;

    Ok(())
}
