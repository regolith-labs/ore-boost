use ore_boost_api::prelude::*;
use steel::*;

/// Deposit adds tokens to a stake account.
pub fn process_deposit(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    panic!("Program is in withdraw-only mode for migration to v3.");
}
