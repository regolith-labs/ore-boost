use num_enum::IntoPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum BoostError {
    #[error("Dummy")]
    Dummy = 0,
}

impl From<BoostError> for ProgramError {
    fn from(e: BoostError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
