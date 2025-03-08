use ore_boost_api::prelude::*;
use steel::*;

#[tokio::test]
async fn test_accumulate_rewards() {
    // Amount to reward each round
    let reward_amount = 100;

    // Create a boost with initial state
    let mut boost = Boost {
        expires_at: 0,
        mint: Pubkey::default(),
        multiplier: 1,
        rewards_factor: Numeric::ZERO,
        total_deposits: 0, // Start at 100
        total_stakers: 0,
    };

    // Create a stake account with 100 balance
    let mut stake1 = Stake {
        authority: Pubkey::default(),
        balance: 100,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_rewards_factor: boost.rewards_factor,
        rewards: 0,
    };
    boost.total_deposits += 100;

    // Add 100 rewards to boost
    boost.rewards_factor += Numeric::from_fraction(reward_amount, boost.total_deposits); // 600 rewards total

    // Create a second stake account with a 150 balance
    let mut stake2 = Stake {
        authority: Pubkey::default(),
        balance: 150,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_rewards_factor: boost.rewards_factor,
        rewards: 0,
    };
    boost.total_deposits += 150;

    // Add 100 rewards to boost
    boost.rewards_factor += Numeric::from_fraction(reward_amount, boost.total_deposits);

    // Create a third stake account with a 50 balance
    let mut stake3 = Stake {
        authority: Pubkey::default(),
        balance: 50,
        boost: Pubkey::default(),
        last_claim_at: 0,
        last_deposit_at: 0,
        last_rewards_factor: boost.rewards_factor,
        rewards: 0,
    };
    boost.total_deposits += 50;

    // Add 100 rewards to boost
    boost.rewards_factor += Numeric::from_fraction(reward_amount, boost.total_deposits);

    // Accumulate rewards for each stake
    stake1.accumulate_rewards(&boost);
    stake2.accumulate_rewards(&boost);
    stake3.accumulate_rewards(&boost);

    // Verify rewards are distributed proportionally
    assert_eq!(stake1.rewards, 173); // (1.0 of 100) + (0.40 of 100) + (0.33 of 100) = 173
    assert_eq!(stake2.rewards, 109); //                (0.60 of 100) + (0.50 of 100) = 109
    assert_eq!(stake3.rewards, 16); //                                 (0.16 of 100) = 16

    // Total rewards should equal original amount
    assert!(stake1.rewards + stake2.rewards + stake3.rewards <= 300);
}
