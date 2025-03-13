use ore_api::state::Proof;
use ore_boost_api::{
    consts::BOOST,
    state::{Boost, Checkpoint, Stake},
};
use steel::*;

/// Rebase checkpoints a stake account, committing pending stake, and updating claimable rewards.
pub fn process_rebase(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let clock = Clock::get()?;
    let [signer_info, boost_info, boost_proof_info, boost_rewards_info, checkpoint_info, stake_info, treasury_info, treasury_tokens_info, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    let boost_proof = boost_proof_info
        .is_writable()?
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *boost_info.key)?;
    boost_rewards_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, &ore_api::consts::MINT_ADDRESS)?;
    let checkpoint = checkpoint_info
        .as_account_mut::<Checkpoint>(&ore_boost_api::ID)?
        .assert_mut(|c| c.boost == *boost_info.key)?
        .assert_mut(|c| clock.unix_timestamp > c.ts)?;
    ore_program.is_program(&ore_api::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Kickoff checkpoint.
    if checkpoint.current_id == 0 {
        // Lock the boost.
        boost.locked = 1;

        // Record the total rewards to distribute.
        checkpoint.total_rewards = boost_proof.balance;

        // Claim staking rewards for this boost.
        invoke_signed(
            &ore_api::sdk::claim(
                *boost_info.key,
                *boost_rewards_info.key,
                checkpoint.total_rewards,
            ),
            &[
                boost_info.clone(),
                boost_rewards_info.clone(),
                boost_proof_info.clone(),
                treasury_info.clone(),
                treasury_tokens_info.clone(),
                token_program.clone(),
                ore_program.clone(),
            ],
            &ore_boost_api::ID,
            &[BOOST, boost.mint.as_ref()],
        )?;
    }

    // Process stake account if it exists.
    if boost.total_stakers > 0 {
        // Load stake account.
        let stake = stake_info
            .as_account_mut::<Stake>(&ore_boost_api::ID)?
            .assert_mut(|s| s.boost == *boost_info.key)?;

        // Silently return if the provided stake account is not the expected stake account.
        if stake.id != checkpoint.current_id {
            return Ok(());
        }

        // Distribute staker rewards according to stake weight.
        if checkpoint.current_id < checkpoint.total_stakers && boost.total_deposits > 0 {
            let rewards: u64 = (checkpoint.total_rewards as u128)
                .checked_mul(stake.balance as u128)
                .unwrap()
                .checked_div(boost.total_deposits as u128)
                .unwrap() as u64;
            stake.rewards = stake.rewards.checked_add(rewards).unwrap();
        }

        // Commit pending stake.
        checkpoint.total_pending_deposits = checkpoint
            .total_pending_deposits
            .checked_add(stake.balance_pending)
            .unwrap();
        stake.balance = stake.balance.checked_add(stake.balance_pending).unwrap();
        stake.balance_pending = 0;
    }

    // Increment the current id.
    checkpoint.current_id = checkpoint.current_id.checked_add(1).unwrap();

    // Finalize the checkpoint.
    if checkpoint.current_id >= boost.total_stakers {
        boost.locked = 0;
        boost.total_deposits = boost
            .total_deposits
            .checked_add(checkpoint.total_pending_deposits)
            .unwrap();
        checkpoint.current_id = 0;
        checkpoint.total_pending_deposits = 0;
        checkpoint.total_rewards = 0;
        checkpoint.total_stakers = boost.total_stakers;
        checkpoint.ts = clock.unix_timestamp;
    }

    Ok(())
}
