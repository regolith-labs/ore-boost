use steel::*;

use super::BoostAccount;

/// Checkpoint holds the checkpoint state of a particular boost.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Checkpoint {
    /// The boost this checkpoint is associated with.
    pub boost: Pubkey,

    /// The id of the next staker to be processed.
    pub current_id: u64,

    /// The total amount of pending deposits in this checkpoint.
    pub total_pending_deposits: u64,

    /// The number of total stakers in this checkpoint.
    pub total_stakers: u64,

    /// The total amount of rewards to distribute in this checkpoint.
    pub total_rewards: u64,

    /// The timestamp of when the last checkpoint finished.
    pub ts: i64,
}

account!(BoostAccount, Checkpoint);

