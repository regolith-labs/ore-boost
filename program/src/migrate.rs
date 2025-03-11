use ore_boost_api::{
    consts::{BOOST, STAKE},
    state::{Boost, Config, Stake},
};
use solana_program::system_program;
use steel::*;

/// Open creates a new stake account.
pub fn process_migrate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, authority_info, config_info, payer_info, boost_info, boost_v1_info, boost_deposits_info, boost_deposits_v1_info, boost_rewards_info, boost_rewards_v1_info, mint_info, stake_info, stake_v1_info, ore_boost_v1_program, system_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    authority_info.is_writable()?;
    config_info
        .as_account_mut::<Config>(&ore_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?; // Migration can only be called by the admin
    payer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    let boost_v1 = boost_v1_info
        .as_account::<ore_boost_api_v1::state::Boost>(&ore_boost_api_v1::ID)? // TODO Parsing
        .assert(|b| b.mint == *mint_info.key)?;
    boost_deposits_info.as_associated_token_account(&boost_info.key, mint_info.key)?;
    let boost_deposits_v1 =
        boost_deposits_v1_info.as_associated_token_account(&boost_v1_info.key, mint_info.key)?;
    boost_rewards_info
        .as_associated_token_account(&boost_info.key, &ore_api::consts::MINT_ADDRESS)?;
    let boost_rewards_v1 = boost_rewards_v1_info
        .as_associated_token_account(&boost_v1_info.key, &ore_api::consts::MINT_ADDRESS)?;
    mint_info.as_mint()?;
    stake_info.is_writable()?.has_seeds(
        &[STAKE, authority_info.key.as_ref(), boost_info.key.as_ref()],
        &ore_boost_api::ID,
    )?;
    let stake_v1 = stake_v1_info
        .as_account::<ore_boost_api_v1::state::Stake>(&ore_boost_api_v1::ID)?
        .assert(|s| s.authority == *authority_info.key)?
        .assert(|s| s.boost == *boost_v1_info.key)?; // TODO Parsing
    ore_boost_v1_program.is_program(&ore_boost_api_v1::ID)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Exit early if stake account is processed out of order.
    if stake_v1.id != boost.total_stakers {
        return Ok(());
    }
    if !stake_info.data_is_empty() {
        return Ok(());
    }

    // Initialize the stake account.
    create_program_account::<Stake>(
        stake_info,
        system_program,
        payer_info,
        &ore_boost_api::ID,
        &[STAKE, authority_info.key.as_ref(), boost_info.key.as_ref()],
    )?;
    let stake = stake_info.as_account_mut::<Stake>(&ore_boost_api::ID)?;
    stake.authority = *authority_info.key;
    stake.balance = 0;
    stake.boost = *boost_info.key;
    stake.last_claim_at = clock.unix_timestamp;
    stake.last_deposit_at = clock.unix_timestamp;
    stake.last_rewards_factor = Numeric::ZERO;
    stake.rewards = 0;

    // Accumulate raw rewards into boost rewards factor, to be divided by total deposits at end of migration.
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
        boost.rewards_factor = boost.rewards_factor / Numeric::from_u64(boost.total_deposits);
    }

    // Get pre transfer balances
    // let pre_boost_deposits_balance = boost_deposits_v1.amount();
    // let pre_boost_rewards_balance = boost_rewards_v1.amount();

    // Migrate deposits and rewards assets.
    // invoke_signed(
    //     &ore_boost_api_v1::sdk::migrate(
    //         *signer_info.key,
    //         *authority_info.key,
    //         boost.mint,
    //         *boost_info.key,
    //         *stake_info.key,
    //     ),
    //     &[
    //         signer_info.clone(),
    //         authority_info.clone(),
    //         boost_v1_info.clone(),
    //         boost_info.clone(),
    //         boost_deposits_v1_info.clone(),
    //         boost_deposits_info.clone(),
    //         boost_rewards_v1_info.clone(),
    //         boost_rewards_info.clone(),
    //         mint_info.clone(),
    //         stake_v1_info.clone(),
    //         stake_info.clone(),
    //         system_program.clone(),
    //         token_program.clone(),
    //     ],
    //     &ore_boost_api::ID,
    //     &[BOOST, boost.mint.as_ref()],
    // )?;

    // Assert stake v1 balance and rewards are 0
    // let stake_v1 =
    //     stake_v1_info.as_account::<ore_boost_api_v1::state::Stake>(&ore_boost_api_v1::ID)?;
    // assert_eq!(stake_v1.balance, 0);
    // assert_eq!(stake_v1.rewards, 0);

    // Assert boost v1 deposits and rewards are reduced by stake v1 balance and rewards
    // let boost_deposits_v1 =
    //     boost_deposits_v1_info.as_associated_token_account(&boost_v1_info.key, &boost.mint)?;
    // let boost_rewards_v1 = boost_rewards_v1_info
    //     .as_associated_token_account(&boost_v1_info.key, &ore_api::consts::MINT_ADDRESS)?;
    // assert_eq!(
    //     boost_deposits_v1.amount(),
    //     pre_boost_deposits_balance - stake_v1.balance
    // );
    // assert_eq!(
    //     boost_rewards_v1.amount(),
    //     pre_boost_rewards_balance - stake_v1.rewards
    // );

    Ok(())
}
