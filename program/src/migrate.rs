use ore_boost_api::{
    consts::STAKE,
    state::{Boost, Config, Stake},
};
use solana_program::system_program;
use steel::*;

/// Open creates a new stake account.
pub fn process_migrate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, config_info, payer_info, boost_info, boost_v1_info, mint_info, stake_info, stake_v1_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    config_info
        .as_account_mut::<Config>(&ore_boost_api::ID)?
        .assert_mut(|c| c.authority == *signer_info.key)?; // Migration can only be called by the admin
    payer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    let boost_v1 = boost_v1_info
        .as_account::<ore_boost_api_v1::state::Boost>(&ore_boost_api_v1::ID)? // TODO Parsing
        .assert(|b| b.mint == *mint_info.key)?;
    mint_info.as_mint()?;
    stake_info.is_empty()?.is_writable()?.has_seeds(
        &[STAKE, signer_info.key.as_ref(), boost_info.key.as_ref()],
        &ore_boost_api::ID,
    )?;
    let stake_v1 = stake_v1_info
        .as_account::<ore_boost_api_v1::state::Stake>(&ore_boost_api_v1::ID)?
        .assert(|s| s.id == boost.total_stakers)?; // TODO Parsing
    system_program.is_program(&system_program::ID)?;

    // Initialize the stake account.
    create_program_account::<Stake>(
        stake_info,
        system_program,
        payer_info,
        &ore_boost_api::ID,
        &[STAKE, signer_info.key.as_ref(), boost_info.key.as_ref()],
    )?;
    let clock = Clock::get()?;
    let stake = stake_info.as_account_mut::<Stake>(&ore_boost_api::ID)?;
    stake.authority = *signer_info.key;
    stake.balance = 0;
    stake.boost = *boost_info.key;
    stake.last_rewards_factor = boost.rewards_factor;
    stake.last_claim_at = clock.unix_timestamp;
    stake.last_deposit_at = clock.unix_timestamp;
    stake.rewards = 0;

    // Accumulate raw rewards into rewards factor, to be divided by total deposits at end of migration.
    boost.rewards_factor += Numeric::from_u64(stake_v1.rewards);

    // Update boost totals.
    boost.total_deposits += stake_v1.balance;
    boost.total_stakers += 1;

    // Update stake balance.
    // Stake rewards balance is not updated here since it is captured by the boost rewards factor.
    // It will be updated on the next claim/deposit/withdraw.
    stake.balance = stake_v1.balance;

    // Finish migration.
    if boost.total_stakers == boost_v1.total_stakers {
        let rewards = boost.rewards_factor.to_i80f48().to_num::<u64>();
        boost.rewards_factor += Numeric::from_fraction(rewards, boost.total_deposits);
    }

    // TODO: Migrate deposits and rewards assets.
    // invoke_signed(
    // &ore_boost_api_v1::sdk::migrate(
    //     *boost_info.key,
    //     *stake_info.key,
    //     stake_v1.balance,
    // ),
    // )?;

    Ok(())
}
