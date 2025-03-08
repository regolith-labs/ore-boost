use steel::*;

use super::BoostAccount;

/// Boost tracks the rewards multiplier and total stake balances of an incentive.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Boost {
    /// The unix timestamp this boost expires.
    pub expires_at: i64,

    /// The mint address of the token associated with this boost.
    pub mint: Pubkey,

    /// The rewards multiplier associated with this boost.
    pub multiplier: u64,

    /// The cumulative rewards currently collected by this boost.
    pub rewards_factor: Numeric,

    // The total amount of stake deposited in this boost.
    pub total_deposits: u64,

    /// The number of stakers in this boost.
    pub total_stakers: u64,
}

account!(BoostAccount, Boost);
