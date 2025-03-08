use steel::*;

use super::BoostAccount;

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

    /// The last boost rewards value this stake account has recorded.
    pub last_boost_rewards: Numeric,

    /// The timestamp of the last time stake was claimed.
    pub last_claim_at: i64,

    /// The timestamp of the last time stake was added to this account.
    pub last_deposit_at: i64,

    /// The amount of rewards claimable by this staker.
    pub rewards: u64,
}

account!(BoostAccount, Stake);
