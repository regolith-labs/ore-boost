mod close;
mod deposit;
mod initialize;
mod new;
mod open;
mod update_admin;
mod update_boost;
mod withdraw;

use close::*;
use deposit::*;
use initialize::*;
use new::*;
use open::*;
use update_admin::*;
use update_boost::*;
use withdraw::*;

use ore_boost_api::instruction::*;
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

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
        BoostInstruction::Close => process_close(accounts, data)?,
        BoostInstruction::Open => process_open(accounts, data)?,
        BoostInstruction::Deposit => process_deposit(accounts, data)?,
        BoostInstruction::Withdraw => process_withdraw(accounts, data)?,

        // Admin
        BoostInstruction::Initialize => process_initialize(accounts, data)?,
        BoostInstruction::New => process_new(accounts, data)?,
        BoostInstruction::UpdateAdmin => process_update_admin(accounts, data)?,
        BoostInstruction::UpdateBoost => process_update_boost(accounts, data)?,
    }

    Ok(())
}
