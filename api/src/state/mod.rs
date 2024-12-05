mod boost;
mod config;
mod checkpoint;
mod leaderboard;
mod stake;

pub use boost::*;
pub use config::*;
pub use checkpoint::*;
pub use leaderboard::*;
pub use stake::*;

use steel::*;

use crate::consts::{BOOST, CHECKPOINT, CONFIG, LEADERBOARD, STAKE};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum BoostAccount {
    Boost = 100,
    Config = 101,
    Stake = 102,
    Leaderboard = 103,
    Checkpoint = 104,
}

/// Fetch the PDA of the boost account.
pub fn boost_pda(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BOOST, mint.as_ref()], &crate::id())
}

/// Fetch the PDA of the config account.
pub fn config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG], &crate::id())
}

/// Fetch the PDA of the stake account.
pub fn stake_pda(authority: Pubkey, boost: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[STAKE, authority.as_ref(), boost.as_ref()], &crate::id())
}

/// Fetch the PDA of the leaderboard account.
pub fn leaderboard_pda(boost: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[LEADERBOARD, boost.as_ref()], &crate::id())
}

/// Fetch the PDA of the checkpoint account.
pub fn checkpoint_pda(boost: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CHECKPOINT, boost.as_ref()], &crate::id())
}
