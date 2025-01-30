use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[rustfmt::skip]
pub enum BoostInstruction {
    // User
    Claim = 0,
    Deposit = 1,
    Open = 2,
    Rebase = 3,
    Register = 4,
    Rotate = 5,
    Withdraw = 6,
    RebaseMany = 7,
    CreateStakeLookupTable = 8,
    ExtendStakeLookupTable = 9,
    
    // Admin
    Activate = 100,
    Deactivate = 101,
    Initialize = 102,
    New = 103,
    UpdateAdmin = 104,
    UpdateBoost = 105,
}

impl BoostInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Activate {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Claim {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateStakeLookupTable {
    pub stake_bump: u8,
    pub lut_id: [u8; 8],
    pub lut_slot: [u8; 8],
    pub lut_bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Deactivate {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Deposit {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ExtendStakeLookupTable {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct New {
    pub expires_at: [u8; 8],
    pub multiplier: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Open {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Rebase {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct RebaseMany {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Register {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Rotate {}

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

instruction!(BoostInstruction, Activate);
instruction!(BoostInstruction, Claim);
instruction!(BoostInstruction, CreateStakeLookupTable);
instruction!(BoostInstruction, Deactivate);
instruction!(BoostInstruction, Deposit);
instruction!(BoostInstruction, ExtendStakeLookupTable);
instruction!(BoostInstruction, Initialize);
instruction!(BoostInstruction, New);
instruction!(BoostInstruction, Open);
instruction!(BoostInstruction, Rebase);
instruction!(BoostInstruction, RebaseMany);
instruction!(BoostInstruction, Register);
instruction!(BoostInstruction, Rotate);
instruction!(BoostInstruction, UpdateAdmin);
instruction!(BoostInstruction, UpdateBoost);
instruction!(BoostInstruction, Withdraw);
