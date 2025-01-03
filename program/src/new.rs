use ore_boost_api::{
    consts::{BOOST, CHECKPOINT},
    instruction::New,
    state::{Boost, Checkpoint, Config, Directory},
};
use solana_program::system_program;
use steel::*;

/// New creates a new boost.
pub fn process_new(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = New::try_from_bytes(data)?;
    let expires_at = i64::from_le_bytes(args.expires_at);
    let multiplier = u64::from_le_bytes(args.multiplier);

    // Load accounts.
    let [signer_info, boost_info, boost_tokens_info, boost_rewards_info, checkpoint_info, config_info, directory_info, mint_info, ore_mint_info, proof_info, ore_program, system_program, token_program, associated_token_program, slot_hashes] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info
        .is_writable()?
        .is_empty()?
        .has_seeds(&[BOOST, mint_info.key.as_ref()], &ore_boost_api::ID)?;
    boost_tokens_info.is_writable()?.is_empty()?;
    boost_rewards_info.is_writable()?.is_empty()?;
    checkpoint_info
        .is_writable()?
        .is_empty()?
        .has_seeds(&[CHECKPOINT, boost_info.key.as_ref()], &ore_boost_api::ID)?;
    config_info
        .as_account::<Config>(&ore_boost_api::ID)?
        .assert(|c| c.authority == *signer_info.key)?;
    let directory = directory_info
        .as_account_mut::<Directory>(&ore_boost_api::ID)?
        .assert_mut(|d| d.len < 256)?;
    mint_info.as_mint()?;
    ore_mint_info.has_address(&ore_api::consts::MINT_ADDRESS)?.as_mint()?;
    proof_info.is_writable()?.is_empty()?;
    ore_program.is_program(&ore_api::ID)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
    slot_hashes.is_sysvar(&sysvar::slot_hashes::ID)?;

    // Add boost to directory.
    directory.boosts[directory.len] = *boost_info.key;
    directory.len += 1;

    // Initialize the boost account.
    create_account::<Boost>(
        boost_info,
        system_program,
        signer_info,
        &ore_boost_api::ID,
        &[BOOST, mint_info.key.as_ref()],
    )?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    boost.expires_at = expires_at;
    boost.mint = *mint_info.key;
    boost.multiplier = multiplier;
    boost.total_stake = 0;

    // Initialize checkpoint account.
    create_account::<Checkpoint>(
        checkpoint_info,
        system_program,
        signer_info,
        &ore_boost_api::ID,
        &[CHECKPOINT, boost_info.key.as_ref()],
    )?;
    let checkpoint = checkpoint_info.as_account_mut::<Checkpoint>(&ore_boost_api::ID)?;
    checkpoint.boost = *boost_info.key;
    checkpoint.current_id = 0;
    checkpoint.total_pending_stake = 0;
    checkpoint.total_rewards = 0;
    checkpoint.total_stakers = 0;
    checkpoint.ts = 0;

    // Open a proof account for this boost.
    invoke_signed(
        &ore_api::sdk::open(
            *boost_info.key, 
            *boost_info.key, 
            *signer_info.key), 
        &[
            boost_info.clone(),
            boost_info.clone(),
            signer_info.clone(),
            proof_info.clone(),
            system_program.clone(),
            slot_hashes.clone(),
            ore_program.clone(),
        ], 
        &ore_boost_api::ID, 
        &[BOOST, mint_info.key.as_ref()]
    )?;

    // Create token account to hold staked tokens.
    create_associated_token_account(
        signer_info,
        boost_info,
        boost_tokens_info,
        mint_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    // Create token account to accumulate staking rewards.
    create_associated_token_account(
        signer_info,
        boost_info,
        boost_rewards_info,
        ore_mint_info,
        system_program,
        token_program,
        associated_token_program,
    )?;
    
    Ok(())
}
