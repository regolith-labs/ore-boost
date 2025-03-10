use ore_api::consts::INITIALIZER_ADDRESS;
use ore_boost_api::{
    consts::BOOST,
    state::{Boost, Stake},
};
use solana_program::system_program;
use steel::*;

/// Migrate migrates a stake account from the v2 boost program to the v3 boost program.
pub fn process_migrate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, authority_info, boost_info, boost_v3_info, boost_deposits_info, boost_deposits_v3_info, boost_rewards_info, boost_rewards_v3_info, mint_info, stake_info, stake_v3_info, system_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.has_address(&INITIALIZER_ADDRESS)?;
    authority_info.is_writable()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    boost_v3_info
        .is_signer()?
        .as_account::<ore_boost_api_v3::state::Boost>(&ore_boost_api_v3::ID)?
        .assert(|b| b.mint == boost.mint)?;
    boost_deposits_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, &boost.mint)?;
    boost_deposits_v3_info
        .is_writable()?
        .as_associated_token_account(boost_v3_info.key, &boost.mint)?;
    boost_rewards_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, &ore_api::consts::MINT_ADDRESS)?;
    boost_rewards_v3_info
        .is_writable()?
        .as_associated_token_account(boost_v3_info.key, &ore_api::consts::MINT_ADDRESS)?;
    mint_info.as_mint()?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *authority_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    stake_v3_info
        .as_account::<ore_boost_api_v3::state::Stake>(&ore_boost_api_v3::ID)?
        .assert(|s| s.authority == stake.authority)?
        .assert(|s| s.boost == *signer_info.key)?
        .assert(|s| s.balance == stake.balance)?
        .assert(|s| s.rewards == stake.rewards)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Migrate deposits.
    transfer_signed(
        boost_info,
        boost_deposits_info,
        boost_deposits_v3_info,
        token_program,
        stake.balance,
        &[BOOST, boost.mint.as_ref()],
    )?;

    // Migrate rewards.
    transfer_signed(
        boost_info,
        boost_rewards_info,
        boost_rewards_v3_info,
        token_program,
        stake.rewards,
        &[BOOST, boost.mint.as_ref()],
    )?;

    // Decrement amounts.
    boost.total_deposits -= stake.balance;
    stake.rewards = 0;
    stake.balance = 0;

    Ok(())
}
