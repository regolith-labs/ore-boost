use steel::*;

use super::BoostAccount;

/// Config holds onto global program variables.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Config {
    /// The admin with authority to create and update boost incentives.
    pub admin: Pubkey,

    /// The list of all boosts available for activation.
    pub boosts: [Pubkey; 256],

    /// The address of the currently active boost.
    pub current: Pubkey,

    /// The number of boosts available in the directory.
    pub len: u64,

    /// The noise used to sample boost activations.
    pub noise: [u8; 32],

    /// The portion of boost rewards stakers should receive (in basis points).
    pub staker_take_rate: u64,

    /// A timestamp of the last boost rotation.
    pub ts: i64,

    /// A buffer for future config variables.
    pub _buffer: [u8; 1024],
}

account!(BoostAccount, Config);
