mod boost;
mod config;
mod checkpoint;
mod directory;
mod reservation;
mod stake;

pub use boost::*;
pub use config::*;
pub use checkpoint::*;
pub use directory::*;
pub use reservation::*;
pub use stake::*;

use steel::*;

use crate::consts::{BOOST, CHECKPOINT, CONFIG, DIRECTORY, RESERVATION, STAKE};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum BoostAccount {
    Boost = 100,
    Checkpoint = 101,
    Config = 102,
    Directory = 103,
    Reservation = 104,
    Stake = 105,
}

/// Fetch the PDA of the boost account.
pub fn boost_pda(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BOOST, mint.as_ref()], &crate::id())
}

/// Fetch the PDA of the checkpoint account.
pub fn checkpoint_pda(boost: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CHECKPOINT, boost.as_ref()], &crate::id())
}

/// Fetch the PDA of the config account.
pub fn config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG], &crate::id())
}

/// Fetch the PDA of the directory account.
pub fn directory_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[DIRECTORY], &crate::id())
}

/// Fetch the PDA of the reservation account.
pub fn reservation_pda(authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[RESERVATION, authority.as_ref()], &crate::id())
}

/// Fetch the PDA of the stake account.
pub fn stake_pda(authority: Pubkey, boost: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[STAKE, authority.as_ref(), boost.as_ref()], &crate::id())
}
