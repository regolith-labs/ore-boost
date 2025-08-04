use steel::*;

use super::BoostAccount;

/// Reserve holds onto the ORE received from the treasury of the ore program.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Reserve {}

account!(BoostAccount, Reserve);
