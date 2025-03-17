use ore_api::state::Proof;
use ore_boost_api::prelude::*;
use steel::*;

#[tokio::test]
async fn test_accumulate_rewards() {
    // Amount to reward each round
    #[allow(deprecated)]
    let mut proof = Proof {
        authority: Pubkey::default(),
        balance: 0,
        challenge: [0; 32],
        last_hash: [0; 32],
        last_hash_at: 0,
        last_stake_at: 0,
        miner: Pubkey::default(),
        total_hashes: 0,
        total_rewards: 0,
    };

    // Create a boost with initial state
    let mut boost = Boost {
        expires_at: 0,
        mint: Pubkey::default(),
        multiplier: 1,
        rewards_factor: Numeric::ZERO,
        total_deposits: 0, // Start at 100
        total_stakers: 3,
        withdraw_fee: 0,
        _buffer: [0; 1024],
    };

    // Create three different stake accounts.
    let mut stake1 = Stake {
        authority: Pubkey::default(),
        balance: 0,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_withdraw_at: 0,
        last_rewards_factor: boost.rewards_factor,
        rewards: 0,
        _buffer: [0; 1024],
    };
    let mut stake2 = Stake {
        authority: Pubkey::default(),
        balance: 0,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_withdraw_at: 0,
        last_rewards_factor: boost.rewards_factor,
        rewards: 0,
        _buffer: [0; 1024],
    };
    let mut stake3 = Stake {
        authority: Pubkey::default(),
        balance: 0,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_withdraw_at: 0,
        last_rewards_factor: boost.rewards_factor,
        rewards: 0,
        _buffer: [0; 1024],
    };

    // Stake account 1 deposits 100
    stake1.accumulate_rewards(&mut boost, &mut proof);
    proof.balance = 0;
    stake1.balance += 100;
    boost.total_deposits += 100;

    // Assume 100 rewards are earned
    proof.balance += 100;

    // Stake account 2 deposits 150
    stake2.accumulate_rewards(&mut boost, &mut proof);
    proof.balance = 0;
    stake2.balance += 150;
    boost.total_deposits += 150;

    // Assume 100 rewards are earned.
    proof.balance += 100;

    // Stake account 3 deposits 50
    stake3.accumulate_rewards(&mut boost, &mut proof);
    proof.balance = 0;
    stake3.balance += 50;
    boost.total_deposits += 50;

    // Assume 100 rewards are earned.
    proof.balance += 100;

    // Assume stake 1 claims rewards.
    stake1.accumulate_rewards(&mut boost, &mut proof);
    proof.balance = 0;

    // Assume stake 2 claims rewards.
    stake2.accumulate_rewards(&mut boost, &mut proof);
    proof.balance = 0;

    // Assume stake 3 claims rewards.
    stake3.accumulate_rewards(&mut boost, &mut proof);
    proof.balance = 0;

    // Verify rewards are distributed proportionally
    assert_eq!(stake1.rewards, 173); // (1.0 of 100) + (0.40 of 100) + (0.33 of 100) = 173
    assert_eq!(stake2.rewards, 109); //                (0.60 of 100) + (0.50 of 100) = 109
    assert_eq!(stake3.rewards, 16); //                                 (0.16 of 100) = 16

    // Total rewards earned should be less than or equal to original amount
    assert!(stake1.rewards + stake2.rewards + stake3.rewards <= 300);
}
