use ore_api::state::Proof;
use ore_boost_api::state::Leaderboard;
use steel::*;

/// Rank adds a miner to the leaderboard
pub fn process_rank(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, leaderboard_info, proof_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let leaderboard = leaderboard_info.as_account_mut::<Leaderboard>(&ore_boost_api::ID)?;

    // Rank the miner on the leaderboard.
    if proof_info.data_is_empty() {
        // Remove miner from leaderboard if proof account is empty.
        leaderboard.remove(*signer_info.key);
    } else {
        // Add miner to leaderboard.
        let proof = proof_info.as_account::<Proof>(&ore_api::ID)?;
        let score = (proof.balance as f64).log2() as u64;
        leaderboard.insert(*proof_info.key, score);
    }

    // Update leaderboard total score.
    leaderboard.total_score = leaderboard.entries.iter().map(|e| e.score).sum();

    Ok(())
} 