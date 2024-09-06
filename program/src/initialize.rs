use std::mem::size_of;

use ore_boost_api::{
    consts::{CONFIG, INITIALIZER_ADDRESS},
    instruction::Initialize,
    state::Config,
};
use ore_utils::*;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};

/// Initialize sets up the boost program.
pub fn process_initialize(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Initialize::try_from_bytes(data)?;

    // Load accounts.
    let [signer, config_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_uninitialized_pda(
        config_info,
        &[CONFIG],
        args.config_bump,
        &ore_boost_api::id(),
    )?;
    load_program(system_program, system_program::id())?;

    // Reject if initializer is not valid.
    if signer.key.ne(&INITIALIZER_ADDRESS) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Initialize config account.
    create_pda(
        config_info,
        &ore_boost_api::id(),
        8 + size_of::<Config>(),
        &[CONFIG, &[args.config_bump]],
        system_program,
        signer,
    )?;
    let mut config_data = config_info.data.borrow_mut();
    config_data[0] = Config::discriminator();
    let config = Config::try_from_bytes_mut(&mut config_data)?;
    config.authority = *signer.key;

    Ok(())
}
