use bytemuck::{Pod, Zeroable};
use ore_utils::*;
use solana_program::pubkey::Pubkey;

use crate::consts::CONFIG;

use super::BoostAccount;

/// Config ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Config {
    /// The admin authority with permission to set token multiplers.
    pub authority: Pubkey,
}

/// Fetch the PDA of the config account.
pub fn config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG], &crate::id())
}

account!(BoostAccount, Config);
