use ore_api::{consts::{MINT_ADDRESS, TREASURY_ADDRESS}, state::Proof};
use ore_boost_api::{consts::ROTATION_SAMPLE_COUNT, state::{Directory, Reservation}};
use solana_program::{keccak::hashv, log::sol_log};
use steel::*;

// P(boost) base probability for every miner.
const BASE_RESERVATION_PROBABILITY: u64 = 10;

/// Rotates a reservation to a randomly selected boost in the directory.
pub fn process_rotate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let [signer_info, directory_info, proof_info, reservation_info, treasury_token_info] = accounts else {
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

    // Reset the reservation
    reservation.boost = Pubkey::default();
    reservation.ts = proof.last_hash_at;

    // Sample random number
    let noise = &reservation.noise[..8];
    let mut random_number = u64::from_le_bytes(noise.try_into().unwrap()) as usize;

    // Try to reserve a boost. 
    if directory.len > 0 {
        // Base probability
        // 
        // Gives a fixed % probability of getting a boost to every miner.
        let boost = directory.boosts[random_number % directory.len as usize];
        let noise = &hashv(&[&random_number.to_le_bytes()]).to_bytes()[..8];
        random_number = u64::from_le_bytes(noise.try_into().unwrap()) as usize;
        let b = random_number as u64 % 100;
        if b < BASE_RESERVATION_PROBABILITY {
            reservation.boost = boost;
            sol_log(&format!("Boost: {:?} Attempt: {}", reservation.boost, "base"));
        } else {
            // Proof weighted rotation
            //
            // Each iteration through the loop is a roll of the dice to get a boost. The probability that 
            // any given sample succeeds in getting a boost is proportional to the miner's unclaimed ORE 
            // relative to all the unclaimed ORE in the treasury. The multiplier reservered is
            // chosen uniformly amongst the set of all active multipliers.
            // 
            // For any rotation, the probability of getting a boost is:
            // p(boost) = 1 - (1 - (proof.balance / treasury.balance))^ROTATION_SAMPLE_COUNT
            for i in 0..ROTATION_SAMPLE_COUNT {
                let boost = directory.boosts[random_number % directory.len as usize];
                let noise = &hashv(&[&random_number.to_le_bytes()]).to_bytes()[..8];
                random_number = u64::from_le_bytes(noise.try_into().unwrap()) as usize;
                let k = random_number as u64 % treasury_tokens.amount;
                if k < proof.balance {
                    reservation.boost = boost;
                    sol_log(&format!("Boost: {:?} Attempt: {}", reservation.boost, i));
                    break;
                }
            }
        }
    }

    // Update the noise
    reservation.noise = hashv(&[&reservation.noise]).0;

    Ok(())
}
