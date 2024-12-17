use std::mem::size_of;

use ore_boost_api::{consts::RESERVATION_INTERVAL, state::{Boost, Leaderboard}};
use solana_program::slot_hashes::SlotHash;
use steel::*;

/// Rotates a boost reservation for a randomly selected miner on the leaderboard, weighted by their balance.
pub fn process_rotate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let [signer_info, boost_info, leaderboard_info, slot_hashes_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?; 
    let leaderboard = leaderboard_info.as_account::<Leaderboard>(&ore_boost_api::ID)?;
    slot_hashes_sysvar.is_sysvar(&sysvar::slot_hashes::ID)?;

    // If the boost is already reserved, do nothing.
    let clock = Clock::get()?;
    if clock.unix_timestamp < boost.reserved_at + RESERVATION_INTERVAL {
        return Ok(());
    }

    // Use most recent slot hash to sample a number between 0 and the total balance on the leaderboard.
    let last_hash = &slot_hashes_sysvar.data.borrow()[0..size_of::<SlotHash>()];
    let total_balance = leaderboard.total_score;
    if total_balance == 0 {
        return Err(ProgramError::InvalidAccountData);
    }
    let random_bytes = &last_hash[last_hash.len() - 8..];
    let random_number = u64::from_le_bytes(random_bytes.try_into().unwrap());
    let k = random_number % total_balance;

    // Select a proof weighted by their unclaimed ORE balance.
    let mut cumulative_score: u64 = 0;
    for entry in leaderboard.entries.iter() {        
        cumulative_score = cumulative_score.checked_add(entry.score).unwrap();
        if cumulative_score > k {
            boost.reserved_for = entry.address;
            boost.reserved_at = clock.unix_timestamp;
            break;
        }
    }

    Ok(())
}
