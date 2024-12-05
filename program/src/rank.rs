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
    let proof = proof_info.as_account::<Proof>(&ore_api::ID)?;
    
    // TODO Handle case where is already on the leaderboard. 

    // Add miner to leaderboard.
    leaderboard.insert(*proof_info.key, proof.balance);

    // Update leaderboard total balance.
    leaderboard.total_balance = leaderboard.entries.iter().map(|e| e.balance).sum();

    Ok(())
} 