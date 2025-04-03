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
    let mut config = Config {
        admin: Pubkey::default(),
        boosts: [Pubkey::default(); 256],
        len: 2,
        staker_take_rate: 5000,
        total_weight: 3,
        global_rewards_factor: Numeric::ZERO,
        _buffer: [0; 1024],
    };

    // Create a boost with initial state
    let mut boost_a = Boost {
        expires_at: 0,
        mint: Pubkey::default(),
        weight: 1,
        rewards_factor: Numeric::ZERO,
        last_global_rewards_factor: Numeric::ZERO,
        total_deposits: 0, // Start at 100
        total_stakers: 3,
        withdraw_fee: 0,
        _buffer: [0; 1024],
    };
    let mut boost_b = Boost {
        expires_at: 0,
        mint: Pubkey::default(),
        weight: 2,
        rewards_factor: Numeric::ZERO,
        last_global_rewards_factor: Numeric::ZERO,
        total_deposits: 0, // Start at 100
        total_stakers: 3,
        withdraw_fee: 0,
        _buffer: [0; 1024],
    };

    // Create five different stake accounts.
    let mut stake_1a = Stake {
        authority: Pubkey::default(),
        balance: 0,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_withdraw_at: 0,
        last_rewards_factor: boost_a.rewards_factor,
        rewards: 0,
        _buffer: [0; 1024],
    };
    let mut stake_2a = Stake {
        authority: Pubkey::default(),
        balance: 0,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_withdraw_at: 0,
        last_rewards_factor: boost_a.rewards_factor,
        rewards: 0,
        _buffer: [0; 1024],
    };
    let mut stake_3a = Stake {
        authority: Pubkey::default(),
        balance: 0,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_withdraw_at: 0,
        last_rewards_factor: boost_a.rewards_factor,
        rewards: 0,
        _buffer: [0; 1024],
    };
    let mut stake_1b = Stake {
        authority: Pubkey::default(),
        balance: 0,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_withdraw_at: 0,
        last_rewards_factor: boost_b.rewards_factor,
        rewards: 0,
        _buffer: [0; 1024],
    };
    let mut stake_2b = Stake {
        authority: Pubkey::default(),
        balance: 0,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_withdraw_at: 0,
        last_rewards_factor: boost_b.rewards_factor,
        rewards: 0,
        _buffer: [0; 1024],
    };

    // Stake account 1a deposits 100
    let clock = Clock {
        slot: 0,
        epoch_start_timestamp: 0,
        epoch: 0,
        leader_schedule_epoch: 0,
        unix_timestamp: 0,
    };

    // Tx 1: Deposit 100
    stake_1a.deposit(&mut boost_a, &clock, &mut config, &mut proof, 100);
    proof.balance = 0; // Simulate claim

    // Tx 2: Deposit 100
    stake_1b.deposit(&mut boost_b, &clock, &mut config, &mut proof, 100);
    proof.balance = 0; // Simulate claim

    // Simulate 100 rewards are earned
    proof.balance += 100;

    // Tx 3: Deposit 150
    stake_2a.deposit(&mut boost_a, &clock, &mut config, &mut proof, 150);
    proof.balance = 0; // Simulate claim

    // Tx 4: Deposit 150
    stake_2b.deposit(&mut boost_b, &clock, &mut config, &mut proof, 150);
    proof.balance = 0; // Simulate claim

    // Simulate 100 rewards are earned.
    proof.balance += 100;

    // Tx 5: Deposit 50
    stake_3a.deposit(&mut boost_a, &clock, &mut config, &mut proof, 50);
    proof.balance = 0; // Simulate claim

    // Simulate 100 rewards are earned.
    proof.balance += 100;

    // Tx 6: Claim rewards.
    stake_1a.claim(&mut boost_a, &clock, &mut config, &mut proof, 0);
    proof.balance = 0; // Simulate claim

    // Tx 7: Claim rewards.
    stake_2a.claim(&mut boost_a, &clock, &mut config, &mut proof, 0);
    proof.balance = 0; // Simulate claim

    // Tx 8: Claim rewards.
    stake_3a.claim(&mut boost_a, &clock, &mut config, &mut proof, 0);
    proof.balance = 0; // Simulate claim

    // Tx 9: Claim rewards.
    stake_1b.claim(&mut boost_b, &clock, &mut config, &mut proof, 0);
    proof.balance = 0; // Simulate claim

    // Tx 10: Claim rewards.
    stake_2b.claim(&mut boost_b, &clock, &mut config, &mut proof, 0);
    proof.balance = 0; // Simulate claim

    // Verify global rewards factor.
    // Other transactions not included in expected result since they have numerator 0 (proof balance is 0).
    let expected_global_rewards_factor = Numeric::from_fraction(100, 3) // Tx 3
        + Numeric::from_fraction(100, 3) // Tx 5
        + Numeric::from_fraction(100, 3); // Tx 6
    assert_eq!(config.global_rewards_factor, expected_global_rewards_factor);
    assert!(
        (config.global_rewards_factor * Numeric::from_u64(config.total_weight)).to_u64() <= 300
    );

    // Verify boost rewards factors.
    let a = Numeric::from_fraction(1, 3) * Numeric::from_u64(100) / Numeric::from_u64(100);
    let b = Numeric::from_fraction(1, 3) * Numeric::from_u64(100) / Numeric::from_u64(250);
    let c = Numeric::from_fraction(1, 3) * Numeric::from_u64(100) / Numeric::from_u64(300);
    let d = a + b + c;
    assert_eq!(boost_a.rewards_factor, d);
    let a = Numeric::from_fraction(2, 3) * Numeric::from_u64(100) / Numeric::from_u64(100);
    let b = Numeric::from_fraction(2, 3) * Numeric::from_u64(100) / Numeric::from_u64(250);
    let c = Numeric::from_fraction(2, 3) * Numeric::from_u64(100) / Numeric::from_u64(250);
    let d = a + b + c;
    assert_eq!(boost_b.rewards_factor, d);

    // Verify stake rewards.
    assert_eq!(stake_1a.rewards, 57); // (100/100 * 33.333) + (100/250 * 33.333) + (100/300 * 33.333)
    assert_eq!(stake_2a.rewards, 36); // (000/100 * 33.333) + (150/250 * 33.333) + (150/300 * 33.333)
    assert_eq!(stake_3a.rewards, 5); // (000/100 * 33.333) + (000/250 * 33.333) + (050/300 * 33.333)

    assert_eq!(stake_1b.rewards, 119); // (100/100 * 66.666) + (100/250 * 66.666) + (100/250 * 66.666)
    assert_eq!(stake_2b.rewards, 79); // (000/100 * 66.666) + (150/250 * 66.666) + (150/250 * 66.666)

    // Total rewards earned should be less than or equal to original amount
    assert!(
        stake_1a.rewards
            + stake_2a.rewards
            + stake_3a.rewards
            + stake_1b.rewards
            + stake_2b.rewards
            <= 300
    );
}
