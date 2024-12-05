use steel::*;
use super::BoostAccount;

/// Leaderboard tracks the top 32 miners by proof balance.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Leaderboard {    
    /// The sorted entries (sorted by balance descending)
    pub entries: [Entry; 32],

    /// The number of entries currently stored
    pub len: usize,

    /// The total sum of all balances.
    pub total_balance: u64,
}

/// Entry represents a single position in the leaderboard
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Entry {
    /// The proof address.
    pub address: Pubkey,

    /// The proof balance.
    pub balance: u64,
}


impl Leaderboard {
    /// Insert a new entry into the leaderboard, maintaining sort order
    pub fn insert(&mut self, address: Pubkey, balance: u64) {
        // Find insertion point
        let mut insert_at = self.len as usize;
        for i in 0..self.len as usize {
            if balance > self.entries[i].balance {
                insert_at = i;
                break;
            }
        }

        // Don't insert if balance is too small and board is full
        if insert_at >= 32 && self.len >= 32 {
            return;
        }

        // Shift entries down to make room
        if insert_at < self.len as usize {
            for i in ((insert_at + 1)..=self.len as usize).rev() {
                if i >= 32 {
                    continue;
                }
                self.entries[i] = self.entries[i - 1];
            }
        }

        // Insert new entry
        if insert_at < 32 {
            self.entries[insert_at] = Entry { address, balance };
            self.len = (self.len + 1).min(32);
        }
    }

    /// Remove an entry from the leaderboard
    pub fn remove(&mut self, address: Pubkey) {
        // Find entry to remove
        let mut remove_at = None;
        for i in 0..self.len as usize {
            if self.entries[i].address == address {
                remove_at = Some(i);
                break;
            }
        }

        // Shift entries up to fill gap
        if let Some(i) = remove_at {
            for j in i..((self.len as usize) - 1) {
                self.entries[j] = self.entries[j + 1];
            }
            self.len -= 1;
        }
    }
}

account!(BoostAccount, Leaderboard); 