use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum BoostError {
    #[error("This boost is curerntly locked for checkpointing. Withdraws will be opened after the checkpoint is finalized.")]
    BoostLocked,
}

error!(BoostError);
