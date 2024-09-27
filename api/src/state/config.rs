use bytemuck::{Pod, Zeroable};
use ore_utils::*;
use solana_program::pubkey::Pubkey;

use super::BoostAccount;

/// Config ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Config {
    /// The admin authority with permission to set token multiplers.
    pub authority: Pubkey,
}

account!(BoostAccount, Config);
