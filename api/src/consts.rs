use const_crypto::ed25519;
use solana_program::{pubkey, pubkey::Pubkey};

/// The authority allowed to initialize the program.
pub const INITIALIZER_ADDRESS: Pubkey = pubkey!("HBUh9g46wk2X89CvaNN15UmsznP59rh6od1h8JwYAopk");

/// The denominator of boost multipliers for percentage calculations.
pub const BOOST_DENOMINATOR: u64 = 1000;

/// The number of attempts (random samples) a miner gets to reserve a boost multiplier.
pub const ROTATION_SAMPLE_COUNT: u64 = 5;

/// The seed of the boost PDA.
pub const BOOST: &[u8] = b"boost";

/// The seed of the config PDA.
pub const CONFIG: &[u8] = b"config";

/// The seed of the stake PDA.
pub const STAKE: &[u8] = b"stake";

/// The seed of the directory PDA.
pub const DIRECTORY: &[u8] = b"directory";

/// The seed of the checkpoint PDA.
pub const CHECKPOINT: &[u8] = b"checkpoint";

/// The seed of the reservation PDA.
pub const RESERVATION: &[u8] = b"reservation";

/// The time interval between checkpoints (in seconds)
pub const CHECKPOINT_INTERVAL: i64 = 3600; // 1 hour

/// Program ID for const pda derivations
const PROGRAM_ID: [u8; 32] = unsafe { *(&crate::id() as *const Pubkey as *const [u8; 32]) };

/// The address of the config account.
pub const CONFIG_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[CONFIG], &PROGRAM_ID).0);
