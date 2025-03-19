use ore_api::state::Proof;
use steel::*;

use super::{Boost, BoostAccount};

/// Stake tracks the deposits and rewards of a staker.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Stake {
    /// The authority of this stake account.
    pub authority: Pubkey,

    /// The balance of this stake account.
    pub balance: u64,

    /// The boost this stake account is associated with.
    pub boost: Pubkey,

    /// The timestamp of the last time rewards were claimed from this account.
    pub last_claim_at: i64,

    /// The timestamp of the last time stake was added to this account.
    pub last_deposit_at: i64,

    /// The timestamp of the last time stake was withdrawn from this account.
    pub last_withdraw_at: i64,

    /// The boost rewards factor last time rewards were updated on this stake account.
    pub last_rewards_factor: Numeric,

    /// The amount of rewards claimable by this staker.
    pub rewards: u64,

    /// A buffer for future config variables.
    pub _buffer: [u8; 1024],
}

impl Stake {
    // Accumulate staking rewards.
    pub fn accumulate_rewards(&mut self, boost: &mut Boost, proof: &Proof) {
        if boost.total_deposits > 0 {
            boost.rewards_factor += Numeric::from_fraction(proof.balance, boost.total_deposits);
        }
        if boost.rewards_factor > self.last_rewards_factor {
            let accumulated_rewards = boost.rewards_factor - self.last_rewards_factor;
            if accumulated_rewards < Numeric::ZERO {
                panic!("Accumulated rewards is negative");
            }
            let personal_rewards = accumulated_rewards * Numeric::from_u64(self.balance);
            self.rewards += personal_rewards.to_u64();
        }
        self.last_rewards_factor = boost.rewards_factor;
    }
}

account!(BoostAccount, Stake);
