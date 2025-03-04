use ore_api::state::Proof;
use ore_boost_api::state::{Directory, Reservation};
use solana_program::keccak::hashv;
use steel::*;

/// Rotates a reservation to a randomly selected boost in the directory.
pub fn process_rotate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let [signer_info, directory_info, proof_info, reservation_info, _treasury_token_info] =
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

    // Reset the reservation
    reservation.boost = Pubkey::default();
    reservation.ts = proof.last_hash_at;

    // Sample random number
    let noise = &reservation.noise[..8];
    let random_number = u64::from_le_bytes(noise.try_into().unwrap()) as usize;

    // Reserve a boost.
    if directory.len > 0 {
        let boost = directory.boosts[random_number % directory.len as usize];
        reservation.boost = boost;
    }

    // Update the noise
    reservation.noise = hashv(&[&reservation.noise]).0;

    Ok(())
}
