use ore_boost_api::{
    consts::STAKE,
    state::{Boost, Stake},
};
use solana_program::system_program;
use steel::*;

/// Open creates a new stake account.
pub fn process_open(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    panic!("Program is in withdraw-only mode for migration to v3.");
}
