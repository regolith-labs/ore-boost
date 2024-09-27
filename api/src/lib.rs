pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod sdk;
pub mod state;

pub use ore_utils;
use solana_program::declare_id;

declare_id!("5P1kyfBQ2f91Ro3aqMB58JaPMSKNvvbtoyo8wkWYrumz");
