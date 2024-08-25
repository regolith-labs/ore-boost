use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};

use crate::utils::AccountDeserialize;

/// Update ...
pub fn process_update<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    _data: &[u8],
) -> ProgramResult {
    // // Load accounts.
    // let [signer, proof_info, system_program] = accounts else {
    //     return Err(ProgramError::NotEnoughAccountKeys);
    // };
    // load_signer(signer)?;
    // load_proof(proof_info, signer.key, true)?;
    // load_program(system_program, system_program::id())?;

    Ok(())
}
