use ore_boost_api::{
    consts::BOOST,
    instruction::New,
    state::{Boost, Config},
};
use solana_program::system_program;
use steel::*;

/// New creates a new boost.
pub fn process_new(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = New::try_from_bytes(data)?;
    let expires_at = i64::from_le_bytes(args.expires_at);
    let bps = u64::from_le_bytes(args.bps);

    // Load accounts.
    let [signer_info, boost_info, boost_deposits_info, boost_rewards_info, config_info, mint_info, ore_mint_info, proof_info, ore_program, system_program, token_program, associated_token_program, slot_hashes] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info
        .is_writable()?
        .is_empty()?
        .has_seeds(&[BOOST, mint_info.key.as_ref()], &ore_boost_api::ID)?;
    boost_deposits_info.is_writable()?.is_empty()?;
    boost_rewards_info.is_writable()?.is_empty()?;
    let config = config_info
        .as_account_mut::<Config>(&ore_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;
    mint_info.as_mint()?;
    ore_mint_info
        .has_address(&ore_api::consts::MINT_ADDRESS)?
        .as_mint()?;
    proof_info.is_writable()?.is_empty()?;
    ore_program.is_program(&ore_api::ID)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
    slot_hashes.is_sysvar(&sysvar::slot_hashes::ID)?;

    // Add boost to directory.
    config.boosts[config.len as usize] = *boost_info.key;
    config.len += 1;

    // Initialize the boost account.
    create_program_account::<Boost>(
        boost_info,
        system_program,
        signer_info,
        &ore_boost_api::ID,
        &[BOOST, mint_info.key.as_ref()],
    )?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    boost.expires_at = expires_at;
    boost.mint = *mint_info.key;
    boost.bps = bps;
    boost.rewards_factor = Numeric::ZERO;
    boost.total_deposits = 0;
    boost.total_stakers = 0;
    boost.withdraw_fee = 0;
    boost._buffer = [0; 1024];

    // Open a proof account for this boost.
    invoke_signed(
        &ore_api::sdk::open(*boost_info.key, *boost_info.key, *signer_info.key),
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
        &[BOOST, mint_info.key.as_ref()],
    )?;

    // Create token account to hold staked tokens.
    create_associated_token_account(
        signer_info,
        boost_info,
        boost_deposits_info,
        mint_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    // Create token account to accumulate staking rewards.
    //
    // Note: The same token account is reused for both deposits and rewards if the mint is the ORE token.
    // This is because you cannot create two associated token accounts for the same mint.
    if mint_info.key != ore_mint_info.key {
        create_associated_token_account(
            signer_info,
            boost_info,
            boost_rewards_info,
            ore_mint_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    }

    Ok(())
}
