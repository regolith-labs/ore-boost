use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum BoostError {
    #[error("Dummy")]
    Dummy = 0,
}

error!(BoostError);
