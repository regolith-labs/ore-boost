use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use ore_utils::*;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::state::{boost_pda, config_pda, stake_pda};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[rustfmt::skip]
pub enum BoostInstruction {
    // User
    Open = 0,
    Deposit = 1,
    Withdraw = 2,
    
    // Admin
    Initialize = 100,
    New = 101,
    Update = 102,
}

impl BoostInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Deposit {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {
    pub config_bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct New {
    pub bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Open {
    pub stake_bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Update {
    pub multiplier: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Withdraw {
    pub amount: [u8; 8],
}

instruction!(BoostInstruction, Deposit);
instruction!(BoostInstruction, Initialize);
instruction!(BoostInstruction, New);
instruction!(BoostInstruction, Open);
instruction!(BoostInstruction, Update);
instruction!(BoostInstruction, Withdraw);

// Build deposit instruction.
pub fn deposit(signer: Pubkey, mint: Pubkey, amount: u64) -> Instruction {
    let boost_pda = boost_pda(mint);
    let boost_tokens_address =
        spl_associated_token_account::get_associated_token_address(&boost_pda.0, &mint);
    let sender_address = spl_associated_token_account::get_associated_token_address(&signer, &mint);
    let stake_pda = stake_pda(signer, boost_pda.0);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new(boost_tokens_address, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(sender_address, false),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: Deposit {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// Build initialize instruction.
pub fn initialize(signer: Pubkey) -> Instruction {
    let config_pda = config_pda();
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_pda.0, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: Initialize {
            config_bump: config_pda.1,
        }
        .to_bytes(),
    }
}

// Build new instruction.
pub fn new(signer: Pubkey, mint: Pubkey) -> Instruction {
    let boost_pda = boost_pda(mint);
    let boost_tokens_address =
        spl_associated_token_account::get_associated_token_address(&boost_pda.0, &mint);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new(boost_tokens_address, false),
            AccountMeta::new_readonly(config_pda().0, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        ],
        data: New { bump: boost_pda.1 }.to_bytes(),
    }
}

// Build open instruction.
pub fn open(signer: Pubkey, mint: Pubkey) -> Instruction {
    let boost_pda = boost_pda(mint);
    let stake_pda = stake_pda(signer, boost_pda.0);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(boost_pda.0, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: Open {
            stake_bump: stake_pda.1,
        }
        .to_bytes(),
    }
}

// Build update instruction.
pub fn update(signer: Pubkey, boost: Pubkey, multiplier: u64) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost, false),
            AccountMeta::new_readonly(config_pda().0, false),
        ],
        data: Update {
            multiplier: multiplier.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// Build withdraw instruction.
pub fn withdraw(signer: Pubkey, mint: Pubkey, amount: u64) -> Instruction {
    let boost_pda = boost_pda(mint);
    let boost_tokens_address =
        spl_associated_token_account::get_associated_token_address(&boost_pda.0, &mint);
    let beneficiary_address =
        spl_associated_token_account::get_associated_token_address(&signer, &mint);
    let stake_pda = stake_pda(signer, boost_pda.0);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary_address, false),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new(boost_tokens_address, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: Withdraw {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
