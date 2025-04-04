use ore_api::state::Proof;
use ore_boost_api::{consts::CONFIG, state::*};
use solana_program::{log::sol_log, pubkey};
use steel::*;
use sysvar::rent::Rent;

pub fn process_migrate_config(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, config_info, ore_mint_info, proof_info, rewards_info, system_program, ore_program, token_program, associated_token_program, slot_hashes_sysvar] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let (admin, len) = {
        let old_config = config_info
            .as_account::<OldConfig>(&ore_boost_api::ID)?
            .assert(|c| c.admin == *signer_info.key)?;
        (old_config.admin, old_config.len)
    };
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

    // Realloc old config account
    let rent = Rent::get()?;
    let new_size = 8 + std::mem::size_of::<Config>();
    config_info.realloc(new_size, false)?;
    let required_lamports = rent.minimum_balance(new_size);
    let current_lamports = config_info.lamports();
    config_info.send(current_lamports - required_lamports, signer_info);

    // Create a new config account
    let config = config_info.as_account_mut::<Config>(&ore_boost_api::ID)?;
    config.admin = admin;
    config.boosts = [Pubkey::default(); 256];
    config.boosts[0] = pubkey!("D3U1nvrCapUiuCK3T3asBPyeKjYptVA4RTXcDTqNpP14");
    config.boosts[1] = pubkey!("5ksboZUb57ZuwEkRRHCK8s6BpiNABneKndvkowZdvGhy");
    config.boosts[2] = pubkey!("5qVQiZXaRffQUqD4NmJ5EXHBAbmfdABZxUb714cJATQp");
    config.boosts[3] = pubkey!("8BEzwBTDsKWnjgjxi8Cca7ZatPZQhxUMgS8qWzBhDhrC");
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
