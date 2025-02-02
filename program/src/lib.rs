mod activate;
mod claim;
mod create_stake_lookup_table;
mod deactivate;
mod deposit;
mod extend_stake_lookup_table;
mod initialize;
mod new;
mod open;
mod rebase;
mod register;
mod rotate;
mod update_admin;
mod update_boost;
mod withdraw;

use activate::*;
use claim::*;
use create_stake_lookup_table::*;
use deactivate::*;
use deposit::*;
use extend_stake_lookup_table::*;
use initialize::*;
use new::*;
use open::*;
use rebase::*;
use register::*;
use rotate::*;
use update_admin::*;
use update_boost::*;
use withdraw::*;

use ore_boost_api::instruction::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&ore_boost_api::ID, program_id, data)?;

    match ix {
        // User
        BoostInstruction::Claim => process_claim(accounts, data)?,
        BoostInstruction::CreateStakeLookupTable => {
            process_create_stake_lookup_table(accounts, data)?
        }
        BoostInstruction::Deposit => process_deposit(accounts, data)?,
        BoostInstruction::ExtendStakeLookupTable => {
            process_extend_stake_lookup_table(accounts, data)?
        }
        BoostInstruction::Open => process_open(accounts, data)?,
        BoostInstruction::Rebase => process_rebase(accounts, data)?,
        BoostInstruction::Register => process_register(accounts, data)?,
        BoostInstruction::Rotate => process_rotate(accounts, data)?,
        BoostInstruction::Withdraw => process_withdraw(accounts, data)?,

        // Admin
        BoostInstruction::Activate => process_activate(accounts, data)?,
        BoostInstruction::Deactivate => process_deactivate(accounts, data)?,
        BoostInstruction::Initialize => process_initialize(accounts, data)?,
        BoostInstruction::New => process_new(accounts, data)?,
        BoostInstruction::UpdateAdmin => process_update_admin(accounts, data)?,
        BoostInstruction::UpdateBoost => process_update_boost(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
