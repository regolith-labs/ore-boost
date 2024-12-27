use std::mem::size_of;

use ore_api::{
    consts::{MINT_ADDRESS, TREASURY_ADDRESS},
    state::Proof,
};
use ore_boost_api::{
    consts::BOOST_RESERVATION_SCALAR,
    state::{Directory, Reservation},
};
use solana_program::{keccak::hashv, log, slot_hashes::SlotHash};
use steel::*;

/// Rotates a reservation to a randomly selected boost in the directory.
pub fn process_rotate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let [signer_info, directory_info, proof_info, reservation_info, treasury_token_info, slot_hashes_sysvar] =
        accounts
    else {
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
    let treasury_tokens =
        treasury_token_info.as_associated_token_account(&TREASURY_ADDRESS, &MINT_ADDRESS)?;
    slot_hashes_sysvar.is_sysvar(&sysvar::slot_hashes::ID)?;

    // Reset the reservation
    reservation.boost = Pubkey::default();
    reservation.ts = proof.last_hash_at;

    // Sample random number
    let last_hash = &slot_hashes_sysvar.data.borrow()[0..size_of::<SlotHash>()];
    let noise = &last_hash[last_hash.len() - 8..];
    log::sol_log(format!("noise: {:?}", noise).as_str());
    let mut random_number = u64::from_le_bytes(noise.try_into().unwrap()) as usize;
    log::sol_log(format!("random number: {:?}", random_number).as_str());

    // For each boost, try to reserve it
    if directory.len > 0 {
        for roll in 0..BOOST_RESERVATION_SCALAR {
            log::sol_log(format!("roll: {:?}", roll).as_str());
            let boost = directory.boosts[random_number % directory.len];
            let noise = &hashv(&[&random_number.to_le_bytes()]).to_bytes()[..8];
            log::sol_log(format!("noise: {:?}", noise).as_str());
            random_number = u64::from_le_bytes(noise.try_into().unwrap()) as usize;
            log::sol_log(format!("random number: {:?}", random_number).as_str());
            let k = random_number as u64 % treasury_tokens.amount;
            log::sol_log(format!("k: {:?}", k).as_str());
            if k < proof.balance {
                log::sol_log(format!("assigned: {:?}", boost).as_str());
                reservation.boost = boost;
                break;
            } else {
                log::sol_log("next");
            }
        }
    }

    Ok(())
}
