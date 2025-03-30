use steel::*;

use super::BoostAccount;

/// Boost tracks the priority, deposits, and rewards of a staking incentive.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Boost {
    /// The unix timestamp this boost expires.
    pub expires_at: i64,

    /// The mint address of the token associated with this boost.
    pub mint: Pubkey,

    /// The take rate in basis points (1/100th of a percent).
    pub bps: u64,

    /// The cumulative rewards collected by this boost, divided by the total deposits at the time of collection.
    pub rewards_factor: Numeric,

    /// The total amount of stake deposited in this boost.
    pub total_deposits: u64,

    /// The number of stakers in this boost.
    pub total_stakers: u64,

    /// A protocol fee charged for withdrawing from this boost (in basis points).
    pub withdraw_fee: u64,

    /// A buffer for future config variables.
    pub _buffer: [u8; 1024],
}

account!(BoostAccount, Boost);
