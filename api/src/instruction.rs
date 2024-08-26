use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use ore_utils::*;

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
