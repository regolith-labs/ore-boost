use bytemuck::{Pod, Zeroable};
use ore_utils::*;
use solana_program::pubkey::Pubkey;

use crate::consts::STAKE;

use super::BoostAccount;

/// Stake ...
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Stake {
    /// The authority of this stake account.
    pub authority: Pubkey,

    /// The balance of this stake account.
    pub balance: u64,

    /// The boost this stake account is associated with.
    pub boost: Pubkey,

    /// The timestamp of the last time stake was added to this account.
    pub last_stake_at: i64,
}

/// Fetch the PDA of the stake account.
pub fn stake_pda(authority: Pubkey, boost: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[STAKE, authority.as_ref(), boost.as_ref()], &crate::id())
}

account!(BoostAccount, Stake);
