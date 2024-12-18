use std::mem::size_of;

use ore_api::{consts::{MINT_ADDRESS, TREASURY_ADDRESS}, state::Proof};
use ore_boost_api::state::{Directory, Reservation};
use solana_program::slot_hashes::SlotHash;
use steel::*;

/// Rotates a boost reservation for a randomly selected miner on the directory, weighted by their balance.
pub fn process_rotate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let [signer_info, directory_info, proof_info, reservation_info, treasury_token_info, slot_hashes_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let directory = directory_info.as_account::<Directory>(&ore_boost_api::ID)?;
    let proof = proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.miner == *signer_info.key)?;
    let reservation = reservation_info
        .as_account_mut::<Reservation>(&ore_boost_api::ID)?
        .assert_mut(|r| r.authority == *proof_info.key)?
        .assert_mut(|r| r.ts < proof.last_hash_at)?;
    let treasury_tokens = treasury_token_info.as_associated_token_account(&TREASURY_ADDRESS, &MINT_ADDRESS)?;
    slot_hashes_sysvar.is_sysvar(&sysvar::slot_hashes::ID)?;

    // Sample k
    let last_hash = &slot_hashes_sysvar.data.borrow()[0..size_of::<SlotHash>()];
    let random_bytes = &last_hash[last_hash.len() - 8..];
    let random_number = u64::from_le_bytes(random_bytes.try_into().unwrap());
    let k = random_number % treasury_tokens.amount;

    // Rotate the boost
    if k < proof.balance {
        // If miner is selected for boost, assign one at random.
        let i = random_number as usize % directory.len;
        reservation.boost = directory.boosts[i];
    } else {
        // If miner is not selected for boost, clear the reservation.
        reservation.boost = Pubkey::default();
    }

    // Update the last rotation time.
    reservation.ts = proof.last_hash_at;

    Ok(())
}
