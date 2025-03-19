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
    create_program_account::<Config>(
        config_info,
        system_program,
        signer_info,
        &ore_boost_api::ID,
        &[CONFIG],
    )?;
    let config = config_info.as_account_mut::<Config>(&ore_boost_api::ID)?;
    config.admin = *signer_info.key;
    config.boosts = [Pubkey::default(); 256];
    config.current = Pubkey::default();
    config.len = 0;
    config.noise = [0; 32];
    config.staker_take_rate = 5_000;
    config.ts = 0;

    Ok(())
}
