use steel::*;

use crate::{
    instruction::*,
    state::{boost_pda, checkpoint_pda, config_pda, leaderboard_pda, stake_pda},
};

// Build claim instruction.
pub fn claim(signer: Pubkey, mint: Pubkey, amount: u64) -> Instruction {
    let boost_pda = boost_pda(mint);
    let boost_rewards_address = spl_associated_token_account::get_associated_token_address(&boost_pda.0, &mint);
    let beneficiary_address = spl_associated_token_account::get_associated_token_address(&signer, &mint);
    let stake_pda = stake_pda(signer, boost_pda.0);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary_address, false),
            AccountMeta::new_readonly(boost_pda.0, false),
            AccountMeta::new(boost_rewards_address, false),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Claim {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}


// Build close instruction.
pub fn close(signer: Pubkey, mint: Pubkey) -> Instruction {
    let boost_pda = boost_pda(mint);
    let stake_pda = stake_pda(signer, boost_pda.0);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Close {}.to_bytes(),
    }
}


// Build deposit instruction.
pub fn deposit(signer: Pubkey, mint: Pubkey, amount: u64) -> Instruction {
    let boost_pda = boost_pda(mint);
    let boost_tokens_address =
        spl_associated_token_account::get_associated_token_address(&boost_pda.0, &mint);
    let sender_address = spl_associated_token_account::get_associated_token_address(&signer, &mint);
    let stake_pda = stake_pda(signer, boost_pda.0);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new(boost_tokens_address, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(sender_address, false),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Deposit {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// Build initialize instruction.
#[allow(deprecated)]
pub fn initialize(signer: Pubkey) -> Instruction {
    let config_pda = config_pda();
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(config_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {}
        .to_bytes(),
    }
}

// Build new instruction.
#[allow(deprecated)]
pub fn new(signer: Pubkey, mint: Pubkey, expires_at: i64, multiplier: u64) -> Instruction {
    let boost_pda = boost_pda(mint);
    let boost_tokens_address =
        spl_associated_token_account::get_associated_token_address(&boost_pda.0, &mint);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new(boost_tokens_address, false),
            AccountMeta::new_readonly(config_pda().0, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: New {
            expires_at: expires_at.to_le_bytes(),
            multiplier: multiplier.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// Build open instruction.
#[allow(deprecated)]
pub fn open(signer: Pubkey, payer: Pubkey, mint: Pubkey) -> Instruction {
    let boost_pda = boost_pda(mint);
    let stake_pda = stake_pda(signer, boost_pda.0);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(boost_pda.0, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Open {}
        .to_bytes(),
    }
}

// Build rank instruction.
pub fn rank(signer: Pubkey, proof: Pubkey) -> Instruction {
    let leaderboard = leaderboard_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(leaderboard, false),
            AccountMeta::new_readonly(proof, false),
        ],
        data: Rank {}.to_bytes(),
    }
}

// Build rebase instruction.
pub fn rebase(signer: Pubkey, mint: Pubkey) -> Instruction {
    let boost_pda = boost_pda(mint);
    let checkpoint_pda = checkpoint_pda(boost_pda.0);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new(checkpoint_pda.0, false),
            AccountMeta::new_readonly(config_pda().0, false),
        ],
        data: Rebase {}.to_bytes(),
    }
}


// Build reserve instruction.
pub fn reserve(signer: Pubkey, mint: Pubkey) -> Instruction {
    let boost_pda = boost_pda(mint);
    let leaderboard = leaderboard_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new_readonly(leaderboard, false),
            AccountMeta::new_readonly(sysvar::slot_hashes::ID, false),
        ],
        data: Reserve {}.to_bytes(),
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
    let boost_pda = boost_pda(mint);
    let boost_tokens_address =
        spl_associated_token_account::get_associated_token_address(&boost_pda.0, &mint);
    let beneficiary_address =
        spl_associated_token_account::get_associated_token_address(&signer, &mint);
    let stake_pda = stake_pda(signer, boost_pda.0);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary_address, false),
            AccountMeta::new(boost_pda.0, false),
            AccountMeta::new(boost_tokens_address, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Withdraw {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
