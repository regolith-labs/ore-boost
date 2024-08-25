mod initialize;
mod new;
mod update;

use initialize::*;
use new::*;
use update::*;

use ore_boost_api::instruction::*;
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub(crate) use ore_utils as utils;

solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&ore_boost_api::id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match BoostInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        // User
        BoostInstruction::Open => todo!(),
        BoostInstruction::Stake => todo!(),
        BoostInstruction::Unstake => todo!(),

        // Admin
        BoostInstruction::Initialize => process_initialize(accounts, data)?,
        BoostInstruction::New => process_new(accounts, data)?,
        BoostInstruction::Update => process_update(accounts, data)?,
    }

    Ok(())
}
