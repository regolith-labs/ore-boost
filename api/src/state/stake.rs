use bytemuck::{Pod, Zeroable};
use ore_utils::{account, Discriminator};

use super::AccountDiscriminator;

/// Stake ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Stake {
    /// The authority of this stake account.
    pub authority: u64,

    /// The balance of this stake account.
    pub balance: u64,

    /// The timestamp of the last time stake was added to this account.
    pub last_stake_at: i64,
}

impl Discriminator for Stake {
    fn discriminator() -> u8 {
        AccountDiscriminator::Stake.into()
    }
}

account!(Stake);
