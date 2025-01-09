use ore_api::state::Proof;
use ore_boost_api::{consts::RESERVATION, state::Reservation};
use solana_program::{keccak::hashv, log::sol_log};
use steel::*;

/// Registers a reservation account for a miner.
pub fn process_register(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let [signer_info, payer_info, proof_info, reservation_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    sol_log("A");
    signer_info.is_signer()?;
    sol_log("B");
    payer_info.is_signer()?;
    sol_log("C");
    let proof = proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.miner == *signer_info.key)?;
    sol_log("D");
    reservation_info
        .is_writable()?
        .is_empty()?
        .has_seeds(&[RESERVATION, proof_info.key.as_ref()], &ore_boost_api::ID)?;
    sol_log("E");
    system_program.is_program(&solana_program::system_program::ID)?;
    sol_log("F");
    
    // Create the reservation account.
    create_account::<Reservation>(
        reservation_info,
        system_program,
        payer_info,
        &ore_boost_api::ID,
        &[RESERVATION, proof_info.key.as_ref()],
    )?;
    let reservation = reservation_info.as_account_mut::<Reservation>(&ore_boost_api::ID)?;
    reservation.authority = *proof_info.key;
    reservation.boost = Pubkey::default();
    reservation.noise = hashv(&[proof_info.key.as_ref()]).0;
    reservation.ts = proof.last_hash_at;

    Ok(())
}
