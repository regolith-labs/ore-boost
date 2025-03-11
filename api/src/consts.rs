use solana_program::{pubkey, pubkey::Pubkey};

/// The authority allowed to initialize the program.
pub const INITIALIZER_ADDRESS: Pubkey = pubkey!("EqbHxJd7UJDjDnZtbbgjo4egidbNgvSjttn1NHmF4aos");

/// The seed of the boost PDA.
pub const BOOST: &[u8] = b"boost";

/// The seed of the config PDA.
pub const CONFIG: &[u8] = b"config";

/// The seed of the stake PDA.
pub const STAKE: &[u8] = b"stake";

/// Denominator for basis point calculations.
pub const DENOMINATOR_BPS: u64 = 10_000;

/// The denominator of boost multipliers for percentage calculations.
pub const DENOMINATOR_MULTIPLIER: u64 = 1_000;

/// The duration of a boost rotation in seconds.
pub const ROTATION_DURATION: i64 = 90;
