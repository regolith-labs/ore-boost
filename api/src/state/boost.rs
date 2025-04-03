use ore_api::state::Proof;
use steel::*;

use super::{BoostAccount, Config};

/// Boost tracks the priority, deposits, and rewards of a staking incentive.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Boost {
    /// The unix timestamp this boost expires.
    pub expires_at: i64,

    /// The mint address of the token associated with this boost.
    pub mint: Pubkey,

    /// The weight of this boost relative to other boosts.
    pub weight: u64,

    /// The cumulative rewards collected by this boost, divided by the total deposits at the time of collection.
    pub rewards_factor: Numeric,

    /// The aggregate rewards factor the last time rewards were collected.
    pub last_global_rewards_factor: Numeric,

    /// The total amount of stake deposited in this boost.
    pub total_deposits: u64,

    /// The number of stakers in this boost.
    pub total_stakers: u64,

    /// A protocol fee charged for withdrawing from this boost (in basis points).
    pub withdraw_fee: u64,

    /// A buffer for future config variables.
    pub _buffer: [u8; 1024],
}

impl Boost {
    /// Collect boost rewards from the global rewards pool.
    pub fn collect_rewards(&mut self, config: &mut Config, proof: &Proof) {
        if config.total_weight > 0 {
            config.global_rewards_factor +=
                Numeric::from_fraction(proof.balance, config.total_weight);
        }
        if config.global_rewards_factor > self.last_global_rewards_factor {
            let accumulated_rewards =
                config.global_rewards_factor - self.last_global_rewards_factor;
            if accumulated_rewards < Numeric::ZERO {
                panic!("Accumulated rewards is negative");
            }
            let boost_rewards = accumulated_rewards * Numeric::from_u64(self.weight);
            self.rewards_factor += boost_rewards / Numeric::from_u64(self.total_deposits);
        }
        self.last_global_rewards_factor = config.global_rewards_factor;
    }
}

account!(BoostAccount, Boost);
