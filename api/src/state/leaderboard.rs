use steel::*;
use super::BoostAccount;

/// Leaderboard tracks the top 32 miners by proof score (log2 of unclaimed ORE).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Leaderboard {    
    /// The sorted entries (sorted by score descending)
    pub entries: [Entry; 32],

    /// The number of entries currently stored
    pub len: usize,

    /// The total sum of all scores.
    pub total_score: u64,
}

/// Entry represents a single position in the leaderboard
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Entry {
    /// The proof address.
    pub address: Pubkey,

    /// The log2 of the proof score.
    pub score: u64,
}


impl Leaderboard {
    /// Insert a new entry into the leaderboard, maintaining sort order
    pub fn insert(&mut self, address: Pubkey, score: u64) {
        // First remove existing entry if present
        self.remove(address);
        
        // Find insertion point
        let mut insert_at = self.len as usize;
        for i in 0..self.len as usize {
            if score > self.entries[i].score {  // Note: using score instead of score
                insert_at = i;
                break;
            }
        }

        // Don't insert if score is too small and board is full
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
            self.entries[insert_at] = Entry { address, score };
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