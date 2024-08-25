use bytemuck::{Pod, Zeroable};
use ore_utils::{account, Discriminator};

use super::AccountDiscriminator;

/// Boost ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Boost {
    /// The mint address
    pub mint: u64,

    /// The multiplier allocated to this token.
    pub multiplier: u64,

    // The total amount of stake in this boost.
    pub total_stake: u64,
}

impl Discriminator for Boost {
    fn discriminator() -> u8 {
        AccountDiscriminator::Boost.into()
    }
}

account!(Boost);
