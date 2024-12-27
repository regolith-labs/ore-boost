use steel::*;
use super::BoostAccount;

/// Directory ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Directory {    
    /// The list of all boosts.
    pub boosts: [Pubkey; 256],

    /// The number of boosts currently active
    pub len: usize,
}

account!(BoostAccount, Directory); 