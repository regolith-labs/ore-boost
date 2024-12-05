use ore_boost_api::state::{Boost, Checkpoint, Stake};
use steel::*;

/// Rebases ...
pub fn process_rebase(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let clock = Clock::get()?;
    let [signer_info, boost_info, checkpoint_info, stake_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?;
    let checkpoint = checkpoint_info
        .as_account_mut::<Checkpoint>(&ore_boost_api::ID)?
        .assert_mut(|c| c.boost == *boost_info.key)?
        .assert_mut(|c| c.current_id < c.total_stakers)?
        .assert_mut(|c| clock.unix_timestamp > c.ts + 3600)?; // checkpoint every hour
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.boost == *boost_info.key)?
        .assert_mut(|s| s.id == checkpoint.current_id)?;

    // Lock the boost for checkpointing.
    boost.locked = 1;

    // Update staker rewards according to commited stake.
    let rewards = boost.rewards.checked_mul(stake.balance).unwrap().checked_div(boost.total_stake).unwrap();
    stake.rewards = stake.rewards.checked_add(rewards).unwrap();

    // Update checkpoint total pending stake.
    checkpoint.total_pending_stake = checkpoint.total_pending_stake.checked_add(stake.pending_balance).unwrap();
    stake.pending_balance = 0;

    // End the rebase
    if stake.id == checkpoint.total_stakers {
        boost.locked = 0;
        boost.rewards = 0;
        boost.total_stake = boost.total_stake.checked_add(checkpoint.total_pending_stake).unwrap();
        checkpoint.current_id = 0;
        checkpoint.ts = clock.unix_timestamp;
        checkpoint.total_stakers = boost.total_stakers;
    }

    Ok(())
}
