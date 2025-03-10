use steel::*;

/// Open creates a new stake account.
pub fn process_open(_accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    panic!("Program is in withdraw-only mode for migration to v3.");
}
