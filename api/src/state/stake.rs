use steel::*;

use super::BoostAccount;

/// Stake ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Stake {
    /// The authority of this stake account.
    pub authority: Pubkey,

    /// The balance of this stake account.
    pub balance: u64,

    /// The boost this stake account is associated with.
    pub boost: Pubkey,

    /// The timestamp of the last time stake was added to this account.
    pub last_stake_at: i64,
}

account!(BoostAccount, Stake);
