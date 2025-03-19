use ore_api::state::proof_pda;
use steel::*;

use crate::{
    instruction::*,
    state::{boost_pda, config_pda, stake_pda},
};

// Build activate instruction.
pub fn activate(signer: Pubkey, mint: Pubkey) -> Instruction {
    let boost_pda = boost_pda(mint);
    let config_pda = config_pda();
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(boost_pda.0, false),
            AccountMeta::new(config_pda.0, false),
        ],
        data: Activate {}.to_bytes(),
    }
}

// Build claim instruction.
pub fn claim(signer: Pubkey, beneficiary: Pubkey, mint: Pubkey, amount: u64) -> Instruction {
    let boost_address = boost_pda(mint).0;
    let boost_proof_address = proof_pda(boost_address).0;
    let boost_rewards_address = spl_associated_token_account::get_associated_token_address(
        &boost_address,
        &ore_api::consts::MINT_ADDRESS,
    );
    let stake_address = stake_pda(signer, boost_address).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary, false),
            AccountMeta::new_readonly(boost_address, false),
            AccountMeta::new(boost_proof_address, false),
            AccountMeta::new(boost_rewards_address, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(ore_api::consts::TREASURY_ADDRESS, false),
            AccountMeta::new(ore_api::consts::TREASURY_TOKENS_ADDRESS, false),
            AccountMeta::new_readonly(ore_api::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Claim {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// Build deactivate instruction.
pub fn deactivate(signer: Pubkey, mint: Pubkey) -> Instruction {
    let boost_pda = boost_pda(mint);
    let config_pda = config_pda();
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(boost_pda.0, false),
            AccountMeta::new(config_pda.0, false),
        ],
        data: Deactivate {}.to_bytes(),
    }
}

// Build deposit instruction.
pub fn deposit(signer: Pubkey, mint: Pubkey, amount: u64) -> Instruction {
    let boost_address = boost_pda(mint).0;
    let boost_proof_address = proof_pda(boost_address).0;
    let boost_deposits_address =
        spl_associated_token_account::get_associated_token_address(&boost_address, &mint);
    let boost_rewards_address = spl_associated_token_account::get_associated_token_address(
        &boost_address,
        &ore_api::consts::MINT_ADDRESS,
    );
    let sender_address = spl_associated_token_account::get_associated_token_address(&signer, &mint);
    let stake_address = stake_pda(signer, boost_address).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost_address, false),
            AccountMeta::new(boost_deposits_address, false),
            AccountMeta::new(boost_proof_address, false),
            AccountMeta::new(boost_rewards_address, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(sender_address, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(ore_api::consts::TREASURY_ADDRESS, false),
            AccountMeta::new(ore_api::consts::TREASURY_TOKENS_ADDRESS, false),
            AccountMeta::new_readonly(ore_api::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
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
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(config_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

// Build new instruction.
pub fn new(signer: Pubkey, mint: Pubkey, expires_at: i64, multiplier: u64) -> Instruction {
    let boost_pda = boost_pda(mint);
    let boost_deposits_address =
        spl_associated_token_account::get_associated_token_address(&boost_pda.0, &mint);
    let boost_rewards_address = spl_associated_token_account::get_associated_token_address(
        &boost_pda.0,
        &ore_api::consts::MINT_ADDRESS,
    );
    let config_pda = config_pda();
    let proof_pda = proof_pda(boost_pda.0);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new(boost_deposits_address, false),
            AccountMeta::new(boost_rewards_address, false),
            AccountMeta::new(config_pda.0, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(ore_api::consts::MINT_ADDRESS, false),
            AccountMeta::new(proof_pda.0, false),
            AccountMeta::new_readonly(ore_api::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(sysvar::slot_hashes::ID, false),
        ],
        data: New {
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
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Open {}.to_bytes(),
    }
}

// Build rotate instruction.
pub fn rotate(signer: Pubkey) -> Instruction {
    let config_pda = config_pda();
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(config_pda.0, false),
        ],
        data: Rotate {}.to_bytes(),
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
        program_id: crate::ID,
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
    let boost_address = boost_pda(mint).0;
    let boost_proof_address = proof_pda(boost_address).0;
    let boost_deposits_address =
        spl_associated_token_account::get_associated_token_address(&boost_address, &mint);
    let boost_rewards_address = spl_associated_token_account::get_associated_token_address(
        &boost_address,
        &ore_api::consts::MINT_ADDRESS,
    );
    let beneficiary_address =
        spl_associated_token_account::get_associated_token_address(&signer, &mint);
    let stake_address = stake_pda(signer, boost_address).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary_address, false),
            AccountMeta::new(boost_address, false),
            AccountMeta::new(boost_deposits_address, false),
            AccountMeta::new(boost_proof_address, false),
            AccountMeta::new(boost_rewards_address, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(ore_api::consts::TREASURY_ADDRESS, false),
            AccountMeta::new(ore_api::consts::TREASURY_TOKENS_ADDRESS, false),
            AccountMeta::new_readonly(ore_api::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Withdraw {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
