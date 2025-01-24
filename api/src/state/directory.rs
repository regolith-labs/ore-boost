use steel::*;
use super::BoostAccount;

/// Directory holds the list of active boosts.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Directory {    
    /// The list of all boosts.
    pub boosts: [Pubkey; 256],

    /// The number of boosts currently active
    pub len: u64,
}

account!(BoostAccount, Directory); 