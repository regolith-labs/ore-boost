use ore_boost_api::{
    consts::STAKE,
    state::{Boost, Stake},
};
use solana_program::system_program;
use steel::*;

use crate::extend_stake_lookup_table::extend_stake_lookup_table;

/// Open creates a new stake account.
pub fn process_open(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, payer_info, boost_info, mint_info, stake_info, stake_lookup_table_info, lookup_table_info, system_program, lookup_table_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    payer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    mint_info.as_mint()?;
    stake_info.is_empty()?.is_writable()?.has_seeds(
        &[STAKE, signer_info.key.as_ref(), boost_info.key.as_ref()],
        &ore_boost_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize the stake account.
    create_account::<Stake>(
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
    stake.balance_pending = 0;
    stake.boost = *boost_info.key;
    stake.id = boost.total_stakers;
    stake.last_deposit_at = clock.unix_timestamp;
    stake.rewards = 0;

    // Increment the total number of stakers.
    boost.total_stakers = boost.total_stakers.checked_add(1).unwrap();

    // Insert into lookup table.
    extend_stake_lookup_table(
        signer_info,
        boost_info,
        stake_info,
        stake,
        stake_lookup_table_info,
        lookup_table_info,
        system_program,
        lookup_table_program,
    )?;

    Ok(())
}
