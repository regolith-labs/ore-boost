use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use ore_utils::instruction;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[rustfmt::skip]
pub enum BoostInstruction {
    // User
    Open = 0,
    Stake = 1,
    Unstake = 2,
    
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
pub struct InitializeArgs {
    pub bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct NewArgs {
    pub bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UpdateArgs {
    pub multiplier: [u8; 8],
}

instruction!(InitializeArgs);
instruction!(NewArgs);
instruction!(UpdateArgs);

// /// Builds a claim instruction.
// pub fn claim(signer: Pubkey, beneficiary: Pubkey, amount: u64) -> Instruction {
//     let proof = Pubkey::find_program_address(&[PROOF, signer.as_ref()], &crate::id()).0;
//     let treasury_tokens = spl_associated_token_account::get_associated_token_address(
//         &TREASURY_ADDRESS,
//         &MINT_ADDRESS,
//     );
//     Instruction {
//         program_id: crate::id(),
//         accounts: vec![
//             AccountMeta::new(signer, true),
//             AccountMeta::new(beneficiary, false),
//             AccountMeta::new(proof, false),
//             AccountMeta::new_readonly(TREASURY_ADDRESS, false),
//             AccountMeta::new(treasury_tokens, false),
//             AccountMeta::new_readonly(spl_token::id(), false),
//         ],
//         data: [
//             OreInstruction::Claim.to_vec(),
//             ClaimArgs {
//                 amount: amount.to_le_bytes(),
//             }
//             .to_bytes()
//             .to_vec(),
//         ]
//         .concat(),
//     }
// }

// /// Builds a close instruction.
// pub fn close(signer: Pubkey) -> Instruction {
//     let proof_pda = Pubkey::find_program_address(&[PROOF, signer.as_ref()], &crate::id());
//     Instruction {
//         program_id: crate::id(),
//         accounts: vec![
//             AccountMeta::new(signer, true),
//             AccountMeta::new(proof_pda.0, false),
//             AccountMeta::new_readonly(solana_program::system_program::id(), false),
//         ],
//         data: OreInstruction::Close.to_vec(),
//     }
// }
