use ore_api::state::Proof;
use ore_boost_api::{consts::BOOST, state::*};
use solana_program::log::sol_log;
use steel::*;
use sysvar::rent::Rent;

pub fn process_migrate_hotfix(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    let config = config_info
        .as_account::<Config>(&ore_boost_api::ID)?
        .assert(|c| c.admin == *signer_info.key)?;

    // Initialize the new boost account.

    boost.last_rewards_factor = config.rewards_factor;

    // Assert migration was successful
    assert_eq!(boost.last_rewards_factor, config.rewards_factor);

    Ok(())
}
