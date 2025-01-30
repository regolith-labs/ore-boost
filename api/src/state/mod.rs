mod boost;
mod checkpoint;
mod config;
mod directory;
mod reservation;
mod stake;
mod stake_lookup_table;

pub use boost::*;
pub use checkpoint::*;
pub use config::*;
pub use directory::*;
pub use reservation::*;
pub use stake::*;
pub use stake_lookup_table::*;

use steel::*;

use crate::consts::{BOOST, CHECKPOINT, CONFIG, DIRECTORY, RESERVATION, STAKE, STAKE_LOOKUP_TABLE};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum BoostAccount {
    Boost = 100,
    Checkpoint = 101,
    Config = 102,
    Directory = 103,
    Reservation = 104,
    Stake = 105,
    StakeLookupTable = 106,
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

/// Fetch the PDA of the stake lookup table.
pub fn stake_lookup_table_pda(boost: Pubkey, lut_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            STAKE_LOOKUP_TABLE,
            boost.as_ref(),
            lut_id.to_le_bytes().as_slice(),
        ],
        &crate::id(),
    )
}

/// Find the stake lookup table id that a particular stake account belongs to.
///
/// Each lookup stable can hold 256 pubkey addresses.
/// So units of 256 stakers are grouped together into a lookup table,
/// with integer division (trims the remainder).
pub fn find_stake_lookup_table_id(stake_id: u64) -> u64 {
    stake_id / 256
}
