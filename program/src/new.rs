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
    let weight = u64::from_le_bytes(args.weight);

    // Load accounts.
    let [signer_info, boost_info, config_info, deposits_info, mint_info, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info
        .is_writable()?
        .is_empty()?
        .has_seeds(&[BOOST, mint_info.key.as_ref()], &ore_boost_api::ID)?;
    let config = config_info
        .as_account_mut::<Config>(&ore_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;
    deposits_info.is_writable()?.is_empty()?;
    mint_info.as_mint()?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

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
    boost.weight = weight;
    boost.last_rewards_factor = config.rewards_factor;
    boost.rewards_factor = Numeric::ZERO;
    boost.total_deposits = 0;
    boost.total_stakers = 0;
    boost.withdraw_fee = 0;

    // Create token account to hold staked tokens.
    create_associated_token_account(
        signer_info,
        boost_info,
        deposits_info,
        mint_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    Ok(())
}
