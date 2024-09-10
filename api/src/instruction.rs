use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use ore_utils::*;
use solana_program::pubkey::Pubkey;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[rustfmt::skip]
pub enum BoostInstruction {
    // User
    Close = 0,
    Deposit = 1,
    Open = 2,
    Withdraw = 3,
    
    // Admin
    Initialize = 100,
    New = 101,
    UpdateAdmin = 102,
    UpdateBoost = 103,
}

impl BoostInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Close {}

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
    pub expires_at: [u8; 8],
    pub multiplier: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Open {
    pub stake_bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UpdateAdmin {
    pub new_admin: Pubkey,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UpdateBoost {
    pub expires_at: [u8; 8],
    pub multiplier: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Withdraw {
    pub amount: [u8; 8],
}

instruction!(BoostInstruction, Close);
instruction!(BoostInstruction, Deposit);
instruction!(BoostInstruction, Initialize);
instruction!(BoostInstruction, New);
instruction!(BoostInstruction, Open);
instruction!(BoostInstruction, UpdateAdmin);
instruction!(BoostInstruction, UpdateBoost);
instruction!(BoostInstruction, Withdraw);
