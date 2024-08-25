use bytemuck::{Pod, Zeroable};
use ore_utils::{account, Discriminator};

use super::AccountDiscriminator;

/// Config ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Config {
    /// The maximum amount of ORE that is allowed to be allocated across all boosts.
    pub max_boost: u64,

    /// The total ORE currently allocated across all boosts.
    pub total_boost: u64,
}

impl Discriminator for Config {
    fn discriminator() -> u8 {
        AccountDiscriminator::Config.into()
    }
}

account!(Config);
