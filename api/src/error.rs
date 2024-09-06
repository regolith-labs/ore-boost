use num_enum::IntoPrimitive;
use ore_utils::*;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum BoostError {
    #[error("Dummy")]
    Dummy = 0,
}

error!(BoostError);
