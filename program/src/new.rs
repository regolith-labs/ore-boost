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
    let multiplier = u64::from_le_bytes(args.multiplier);
    let expires_at = i64::from_le_bytes(args.expires_at);

    // Load accounts.
    let [signer_info, boost_info, boost_tokens_info, boost_rewards_info, config_info, mint_info, ore_mint_info, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info
        .is_writable()?
        .is_empty()?
        .has_seeds(&[BOOST, mint_info.key.as_ref()], &ore_boost_api::id())?;
    boost_tokens_info.is_writable()?.is_empty()?;
    boost_rewards_info.is_writable()?.is_empty()?;
    config_info
        .as_account::<Config>(&ore_boost_api::ID)?
        .assert(|c| c.authority == *signer_info.key)?;
    mint_info.as_mint()?;
    ore_mint_info.has_address(&ore_api::consts::MINT_ADDRESS)?.as_mint()?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Initialize the boost account.
    create_account::<Boost>(
        boost_info,
        system_program,
        signer_info,
        &ore_boost_api::id(),
        &[BOOST, mint_info.key.as_ref()],
    )?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    boost.expires_at = expires_at;
    boost.mint = *mint_info.key;
    boost.multiplier = multiplier;
    boost.proof = Pubkey::default();
    boost.reserved_at = 0;
    boost.total_stake = 0;

    // Open a proof account owned by this boost.
    invoke_signed(
        &ore_api::sdk::open(
            *boost_info.key, 
            *boost_info.key, 
            *signer_info.key), 
        &[
            boost_info.clone(),
            signer_info.clone(),
        ], 
        &ore_api::ID, 
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

    // Create token account hold yield.
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
