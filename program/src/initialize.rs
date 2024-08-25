use ore_boost_api::instruction::InitializeArgs;
use ore_utils::spl::transfer_signed;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::utils::AccountDeserialize;

/// Initialize ...
pub fn process_initialize(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = InitializeArgs::try_from_bytes(data)?;

    // Load accounts.
    // let [signer, beneficiary_info, proof_info, treasury_info, treasury_tokens_info, token_program] =
    //     accounts
    // else {
    //     return Err(ProgramError::NotEnoughAccountKeys);
    // };
    // load_signer(signer)?;
    // load_token_account(beneficiary_info, None, &MINT_ADDRESS, true)?;
    // load_proof(proof_info, signer.key, true)?;
    // load_treasury(treasury_info, false)?;
    // load_treasury_tokens(treasury_tokens_info, true)?;
    // load_program(token_program, spl_token::id())?;

    Ok(())
}
