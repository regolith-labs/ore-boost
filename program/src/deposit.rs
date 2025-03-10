use steel::*;

/// Deposit adds tokens to a stake account.
pub fn process_deposit(_accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    panic!("Program is in withdraw-only mode for migration to v3.");
}
