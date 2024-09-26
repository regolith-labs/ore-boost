use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{
    instruction::*,
    state::{boost_pda, config_pda, stake_pda},
};

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
            AccountMeta::new(config_pda.0, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: Initialize {
            config_bump: config_pda.1,
        }
        .to_bytes(),
    }
}

// Build new instruction.
pub fn new(signer: Pubkey, mint: Pubkey, expires_at: i64, multiplier: u64) -> Instruction {
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
        data: New {
            bump: boost_pda.1,
            expires_at: expires_at.to_le_bytes(),
            multiplier: multiplier.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// Build open instruction.
pub fn open(signer: Pubkey, payer: Pubkey, mint: Pubkey) -> Instruction {
    let boost_pda = boost_pda(mint);
    let stake_pda = stake_pda(signer, boost_pda.0);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(payer, true),
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

// Build update_boost instruction.
pub fn update_boost(
    signer: Pubkey,
    boost: Pubkey,
    expires_at: i64,
    multiplier: u64,
) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost, false),
            AccountMeta::new_readonly(config_pda().0, false),
        ],
        data: UpdateBoost {
            expires_at: expires_at.to_le_bytes(),
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
