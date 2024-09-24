pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod sdk;
pub mod state;

pub use ore_utils;
use solana_program::declare_id;

declare_id!("4UT1BjrL7EPZuzEAiyGbSrs2xJrfNfCuZ4yWgW5R8Z1a");
