pub mod consts;
pub mod error;
pub mod instruction;
pub mod sdk;
pub mod state;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::state::*;
}

use steel::*;

// TODO New program id
declare_id!("BooST9jdDtrg3THn5fo2D4XAtKtciBRi7H2muxVK4JS");
