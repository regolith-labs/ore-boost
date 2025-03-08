use const_crypto::ed25519;
use solana_program::{pubkey, pubkey::Pubkey};

/// The authority allowed to initialize the program.
pub const INITIALIZER_ADDRESS: Pubkey = pubkey!("HBUh9g46wk2X89CvaNN15UmsznP59rh6od1h8JwYAopk");

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
