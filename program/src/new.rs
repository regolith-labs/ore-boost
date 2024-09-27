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
    let [signer_info, boost_info, boost_tokens_info, config_info, mint_info, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info.is_writable()?.is_empty()?.has_seeds(
        &[BOOST, mint_info.key.as_ref()],
        args.bump,
        &ore_boost_api::id(),
    )?;
    boost_tokens_info.is_writable()?.is_empty()?;
    config_info
        .to_account::<Config>(&ore_boost_api::ID)?
        .check(|c| c.authority == *signer_info.key)?;
    mint_info.to_mint()?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Initialize the boost account.
    create_account::<Boost>(
        boost_info,
        &ore_boost_api::id(),
        &[BOOST, mint_info.key.as_ref(), &[args.bump]],
        system_program,
        signer_info,
    )?;
    let boost = boost_info.to_account_mut::<Boost>(&ore_boost_api::ID)?;
    boost.bump = args.bump as u64;
    boost.mint = *mint_info.key;
    boost.expires_at = expires_at;
    boost.multiplier = multiplier;
    boost.total_stake = 0;

    // Create boost token account.
    create_associated_token_account(
        signer_info,
        boost_info,
        boost_tokens_info,
        mint_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    Ok(())
}
