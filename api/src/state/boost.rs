use bytemuck::{Pod, Zeroable};
use ore_utils::*;
use solana_program::pubkey::Pubkey;

use super::BoostAccount;

/// Boost ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Boost {
    /// The bump used for signing.
    pub bump: u64,

    /// The mint address
    pub mint: Pubkey,

    /// The multiplier allocated to this token.
    pub multiplier: u64,

    // The total amount of stake in this boost.
    pub total_stake: u64,
}

account!(BoostAccount, Boost);