use ore_boost_api::{
    consts::{CONFIG, INITIALIZER_ADDRESS},
    instruction::Initialize,
    state::Config,
};
use solana_program::system_program;
use steel::*;

/// Initialize sets up the boost program.
pub fn process_initialize(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Initialize::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, config_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.has_address(&INITIALIZER_ADDRESS)?;
    config_info.is_writable()?.is_empty()?.has_seeds(
        &[CONFIG],
        args.config_bump,
        &ore_boost_api::id(),
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize config account.
    create_account::<Config>(
        config_info,
        &ore_boost_api::id(),
        &[CONFIG, &[args.config_bump]],
        system_program,
        signer_info,
    )?;
    let config = config_info.to_account_mut::<Config>(&ore_boost_api::ID)?;
    config.authority = *signer_info.key;

    Ok(())
}
