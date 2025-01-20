use steel::*;

use super::BoostAccount;

/// Config holds global program config variables.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Config {
    /// The admin authority with permission to set token multiplers.
    pub authority: Pubkey,
}

account!(BoostAccount, Config);
