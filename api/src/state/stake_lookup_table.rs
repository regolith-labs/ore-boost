use steel::*;

use super::BoostAccount;

/// Stake Lookup Table holds the pubkey of the lookup table
/// that a particular stake (id) belongs to.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct StakeLookupTable {
    /// The address of the lookup table
    pub lookup_table: Pubkey,

    /// The bump used for signing
    pub bump: u8,
}

account!(BoostAccount, StakeLookupTable);
