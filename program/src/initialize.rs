use ore_boost_api::prelude::*;
use steel::*;

/// Initialize sets up the boost program.
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, config_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.has_address(&INITIALIZER_ADDRESS)?;
    config_info
        .is_writable()?
        .is_empty()?
        .has_seeds(&[CONFIG], &ore_boost_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize config account.
    create_account::<Config>(
        config_info,
        system_program,
        signer_info,
        &ore_boost_api::id(),
        &[CONFIG],
    )?;
    let config = config_info.as_account_mut::<Config>(&ore_boost_api::ID)?;
    config.authority = *signer_info.key;


    Ok(())
}
