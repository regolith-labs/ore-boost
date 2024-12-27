use steel::*;

use super::BoostAccount;

/// Boost ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Reservation {
    /// The proof account associated with this reservation.
    pub authority: Pubkey,

    /// The boost this miner is allowed to use.
    pub boost: Pubkey,

    /// A timestamp to ensure only one rotation is executed per solution.
    pub ts: i64,
}

account!(BoostAccount, Reservation);
