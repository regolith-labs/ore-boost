use ore_api::state::Proof;
use ore_boost_api::{consts::CONFIG, state::*};
use steel::*;

pub fn process_migrate_config(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, config_info, ore_mint_info, proof_info, rewards_info, system_program, ore_program, token_program, associated_token_program, slot_hashes_sysvar] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let old_config = config_info
        .as_account_mut::<OldConfig>(&ore_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;
    ore_mint_info
        .has_address(&ore_api::consts::MINT_ADDRESS)?
        .as_mint()?;
    proof_info.is_empty()?.is_writable()?;
    rewards_info.is_empty()?.is_writable()?;
    system_program.is_program(&system_program::ID)?;
    ore_program.is_program(&ore_api::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
    slot_hashes_sysvar.is_sysvar(&sysvar::slot_hashes::ID)?;

    // Record old values
    let admin = old_config.admin;
    let boosts = old_config.boosts;
    let len = old_config.len;

    // Close the old config account
    config_info.close(signer_info)?;

    // Create a new config account
    create_program_account::<Config>(
        config_info,
        system_program,
        signer_info,
        &ore_boost_api::ID,
        &[CONFIG],
    )?;
    let config = config_info.as_account_mut::<Config>(&ore_boost_api::ID)?;
    config.admin = admin;
    config.boosts = boosts;
    config.len = len;
    config.rewards_factor = Numeric::ZERO;
    config.take_rate = 5_000;
    config.total_weight = 0;

    // Open a proof for the config account.
    invoke_signed(
        &ore_api::sdk::open(*config_info.key, *config_info.key, *signer_info.key),
        &[
            config_info.clone(),
            config_info.clone(),
            signer_info.clone(),
            proof_info.clone(),
            system_program.clone(),
            slot_hashes_sysvar.clone(),
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

    // Assert migraiton was successful
    assert_eq!(config.admin, admin);
    assert_eq!(config.boosts, boosts);
    assert_eq!(config.len, len);
    assert_eq!(config.take_rate, 5_000);
    assert_eq!(config.total_weight, 0);

    // Assert accounts were created correctly
    rewards_info.as_associated_token_account(config_info.key, ore_mint_info.key)?;
    proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *config_info.key)?;

    Ok(())
}
