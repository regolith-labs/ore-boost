use std::mem::size_of;

use ore_boost_api::{consts::STAKE, instruction::Open, loaders::load_boost, state::Stake};
use ore_utils::*;
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult,
    program_error::ProgramError, system_program, sysvar::Sysvar,
};

/// Open creates a new stake account.
pub fn process_open(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Open::try_from_bytes(data)?;

    // Load accounts.
    let [signer, payer, boost_info, mint_info, stake_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_signer(payer)?;
    load_boost(boost_info, mint_info.key, false)?;
    load_uninitialized_pda(
        stake_info,
        &[STAKE, signer.key.as_ref(), boost_info.key.as_ref()],
        args.stake_bump,
        &ore_boost_api::id(),
    )?;
    load_any_mint(mint_info, false)?;
    load_program(system_program, system_program::id())?;

    // Get clock
    let clock = Clock::get().unwrap();

    // Initialize the stake account.
    create_pda(
        stake_info,
        &ore_boost_api::id(),
        8 + size_of::<Stake>(),
        &[
            STAKE,
            signer.key.as_ref(),
            boost_info.key.as_ref(),
            &[args.stake_bump],
        ],
        system_program,
        payer,
    )?;
    let mut stake_data = stake_info.data.borrow_mut();
    stake_data[0] = Stake::discriminator();
    let stake = Stake::try_from_bytes_mut(&mut stake_data)?;
    stake.authority = *signer.key;
    stake.balance = 0;
    stake.boost = *boost_info.key;
    stake.last_stake_at = clock.unix_timestamp;

    Ok(())
}
