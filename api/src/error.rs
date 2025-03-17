use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum BoostError {
    #[error("Dummy error")]
    Dummy,
}

error!(BoostError);
