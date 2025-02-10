use std::mem::size_of;

use ore_api::state::Proof;
use ore_boost_api::{consts::RESERVATION, state::Reservation};
use solana_program::{keccak::hashv, slot_hashes::SlotHash};
use steel::*;

/// Registers a reservation account for a miner.
pub fn process_register(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let [signer_info, payer_info, proof_info, reservation_info, system_program, slot_hashes_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    payer_info.is_signer()?;
    let proof = proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.miner == *signer_info.key)?;
    reservation_info
        .is_writable()?
        .is_empty()?
        .has_seeds(&[RESERVATION, proof_info.key.as_ref()], &ore_boost_api::ID)?;
    system_program.is_program(&solana_program::system_program::ID)?;
    slot_hashes_sysvar.is_sysvar(&sysvar::slot_hashes::ID)?;
    
    // Create the reservation account.
    create_program_account::<Reservation>(
        reservation_info,
        system_program,
        payer_info,
        &ore_boost_api::ID,
        &[RESERVATION, proof_info.key.as_ref()],
    )?;
    let reservation = reservation_info.as_account_mut::<Reservation>(&ore_boost_api::ID)?;
    reservation.authority = *proof_info.key;
    reservation.boost = Pubkey::default();
    reservation.noise = hashv(&[
        proof_info.key.as_ref(), 
        &slot_hashes_sysvar.data.borrow()[0..size_of::<SlotHash>()],
    ]).0;
    reservation.ts = proof.last_hash_at;

    Ok(())
}
