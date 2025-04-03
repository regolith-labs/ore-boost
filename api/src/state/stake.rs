use ore_api::state::Proof;
use steel::*;

use super::{Boost, BoostAccount, Config};

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
    /// Claim rewards.
    pub fn claim(
        &mut self,
        amount: u64,
        boost: &mut Boost,
        clock: &Clock,
        config: &mut Config,
        proof: &Proof,
    ) -> u64 {
        self.collect_rewards(boost, config, &proof);
        let amount = amount.min(self.rewards);
        self.last_claim_at = clock.unix_timestamp;
        self.rewards -= amount;
        amount
    }

    /// Deposit into the boost.
    pub fn deposit(
        &mut self,
        amount: u64,
        boost: &mut Boost,
        clock: &Clock,
        config: &mut Config,
        proof: &Proof,
        sender: &TokenAccount,
    ) -> u64 {
        self.collect_rewards(boost, config, &proof);
        let amount = amount.min(sender.amount());
        boost.total_deposits += amount;
        self.balance += amount;
        self.last_deposit_at = clock.unix_timestamp;
        amount
    }

    /// Withdraw from the boost.
    pub fn withdraw(
        &mut self,
        amount: u64,
        boost: &mut Boost,
        clock: &Clock,
        config: &mut Config,
        proof: &Proof,
    ) -> u64 {
        self.collect_rewards(boost, config, &proof);
        let amount = amount.min(self.balance);
        self.balance -= amount;
        self.last_withdraw_at = clock.unix_timestamp;
        boost.total_deposits -= amount;
        amount
    }

    // Collect staking rewards.
    pub fn collect_rewards(&mut self, boost: &mut Boost, config: &mut Config, proof: &Proof) {
        // Update the boost rewards factor.
        boost.collect_rewards(config, proof);

        // Accumulate stake-weighted boost rewards into the stake account.
        if boost.rewards_factor > self.last_rewards_factor {
            let accumulated_rewards = boost.rewards_factor - self.last_rewards_factor;
            if accumulated_rewards < Numeric::ZERO {
                panic!("Accumulated rewards is negative");
            }
            let personal_rewards = accumulated_rewards * Numeric::from_u64(self.balance);
            self.rewards += personal_rewards.to_u64();
        }

        // Update this stake account's last seen rewards factor.
        self.last_rewards_factor = boost.rewards_factor;
    }
}

account!(BoostAccount, Stake);
