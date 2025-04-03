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

    /// The number of boosts available in the directory.
    pub len: u64,

    /// The portion of rewards boost stakers should receive (in basis points).
    pub staker_take_rate: u64,

    /// The total weight of all boosts.
    pub total_weight: u64,

    /// The cumulative rewards collected by all boosts, divided by the total weight at the time of collection.
    pub global_rewards_factor: Numeric,

    /// A buffer for future config variables.
    pub _buffer: [u8; 1024],
}

account!(BoostAccount, Config);
