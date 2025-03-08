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

    /// The id of this stake account.
    pub id: u64,

    /// The cumulative rewards on the boost account last time rewards were updated on this stake account.
    pub last_boost_cumulative_rewards: Numeric,

    /// The timestamp of the last time stake was claimed.
    pub last_claim_at: i64,

    /// The timestamp of the last time stake was added to this account.
    pub last_deposit_at: i64,

    /// The amount of rewards claimable by this staker.
    pub rewards: u64,
}

impl Stake {
    // Accumulate personal stake rewards.
    pub fn accumulate_rewards(&mut self, boost: &Boost, clock: &Clock) {
        if self.balance > 0 && boost.rewards_cumulative > self.last_boost_cumulative_rewards {
            let accumulated_rewards = boost.rewards_cumulative - self.last_boost_cumulative_rewards;
            let personal_rewards = accumulated_rewards * Numeric::from_u64(self.balance);
            self.rewards += personal_rewards.floor().to_i80f48().to_num::<u64>();
            self.last_claim_at = clock.unix_timestamp;
            self.last_boost_cumulative_rewards = boost.rewards_cumulative;
        }
    }
}

account!(BoostAccount, Stake);
