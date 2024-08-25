mod boost;
mod config;
mod stake;

pub use boost::*;
pub use config::*;
pub use stake::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountDiscriminator {
    Boost = 100,
    Config = 101,
    Stake = 102,
}
