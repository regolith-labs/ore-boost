use ore_boost_api::prelude::*;
use steel::*;

/// Initialize sets up the boost program.
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, config_info, ore_mint_info, proof_info, rewards_info, system_program, ore_program, token_program, associated_token_program, slot_hashes] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.has_address(&INITIALIZER_ADDRESS)?;
    config_info
        .is_writable()?
        .is_empty()?
        .has_seeds(&[CONFIG], &ore_boost_api::ID)?;
    ore_program.is_program(&ore_api::ID)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

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
    config.len = 0;
    config.staker_take_rate = 5_000;

    // Open a proof for the config account.
    invoke_signed(
        &ore_api::sdk::open(*config_info.key, *config_info.key, *signer_info.key),
        &[
            config_info.clone(),
            config_info.clone(),
            signer_info.clone(),
            proof_info.clone(),
            system_program.clone(),
            slot_hashes.clone(),
            ore_program.clone(),
        ],
        &ore_boost_api::ID,
        &[CONFIG],
    )?;

    // Create token account to accumulate staking rewards.
    create_associated_token_account(
        signer_info,
        config_info,
        rewards_info,
        ore_mint_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    Ok(())
}
