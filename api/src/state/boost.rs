use ore_api::state::Proof;
use steel::*;

use super::{BoostAccount, Config};

/// Boost tracks the priority, deposits, and rewards of a staking incentive.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Boost {
    /// The unix timestamp this boost expires.
    pub expires_at: i64,

    /// The config rewards factor the last time rewards were collected by this boost.
    pub last_rewards_factor: Numeric,

    /// The mint address of the token associated with this boost.
    pub mint: Pubkey,

    /// The cumulative rewards collected by this boost, divided by the total deposits at the time of collection.
    pub rewards_factor: Numeric,

    /// The total amount of stake deposited in this boost.
    pub total_deposits: u64,

    /// The number of stakers in this boost.
    pub total_stakers: u64,

    /// The weight of this boost relative to other boosts.
    pub weight: u64,

    /// A protocol fee charged for withdrawing from this boost (in basis points).
    pub withdraw_fee: u64,
}

impl Boost {
    /// Collect weighted rewards from the global rewards pool.
    pub fn collect_rewards(&mut self, config: &mut Config, proof: &Proof) {
        // Increment the global rewards factor
        if config.total_weight > 0 {
            config.rewards_factor += Numeric::from_fraction(proof.balance, config.total_weight);
        }

        // Accumulate weighted rewards into the boost rewards factor
        if config.rewards_factor > self.last_rewards_factor && self.total_deposits > 0 {
            let accumulated_rewards = config.rewards_factor - self.last_rewards_factor;
            if accumulated_rewards < Numeric::ZERO {
                panic!("Accumulated rewards is negative");
            }
            let boost_rewards = accumulated_rewards * Numeric::from_u64(self.weight);
            self.rewards_factor += boost_rewards / Numeric::from_u64(self.total_deposits);
        }

        // Update this boost's last seen rewards factor
        self.last_rewards_factor = config.rewards_factor;
    }
}

account!(BoostAccount, Boost);
