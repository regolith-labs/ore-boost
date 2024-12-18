use steel::*;

use super::BoostAccount;

/// Boost ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Boost {
    /// The bump used for signing.
    pub bump: u64,

    /// The unix timestamp this boost expires.
    pub expires_at: i64,

    /// Flag indicating if this boost is locked for checkpointing.
    pub locked: u64,

    /// The mint address of the token associated with this boost.
    pub mint: Pubkey,

    /// The multiplier allocated to this boost.
    pub multiplier: u64,
    
    // The total amount of stake in this boost.
    pub total_stake: u64,

    /// The number of stakers in this boost.
    pub total_stakers: u64,
}

account!(BoostAccount, Boost);
