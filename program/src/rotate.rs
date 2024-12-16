use std::mem::size_of;

use ore_boost_api::{consts::RESERVATION_INTERVAL, state::{Boost, Leaderboard}};
use solana_program::slot_hashes::SlotHash;
use steel::*;

/// Rotates a boost reservation for a randomly selected miner on the leaderboard, weighted by their balance.
pub fn process_rotate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let clock = Clock::get()?;
    let [signer_info, boost_info, leaderboard_info, slot_hashes_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| clock.unix_timestamp > b.reserved_at + RESERVATION_INTERVAL)?; 
    let leaderboard = leaderboard_info.as_account::<Leaderboard>(&ore_boost_api::ID)?;
    slot_hashes_sysvar.is_sysvar(&sysvar::slot_hashes::ID)?;

    // Use most recent slot hash to sample a number between 0 and the total balance on the leaderboard.
    let last_hash = &slot_hashes_sysvar.data.borrow()[0..size_of::<SlotHash>()];
    let total_balance = leaderboard.total_score;
    if total_balance == 0 {
        return Err(ProgramError::InvalidAccountData);
    }
    let random_bytes = &last_hash[..8];
    let random_number = u64::from_le_bytes(random_bytes.try_into().unwrap());
    let k = random_number % total_balance;

    // Select a proof weighted by their unclaimed ORE balance.
    let mut cumulative_score: u64 = 0;
    let mut selected_proof = None;
    for entry in leaderboard.entries.iter() {
        cumulative_score = cumulative_score.checked_add(entry.score).unwrap();
        if cumulative_score > k {
            selected_proof = Some(entry.address);
            break;
        }
    }
    let proof = selected_proof.ok_or(ProgramError::InvalidAccountData)?;

    // Reserve the boost for the selected proof.
    boost.reserved_for = proof;
    boost.reserved_at = clock.unix_timestamp;

    Ok(())
}
