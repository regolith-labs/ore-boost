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
declare_id!("BoosTyJFPPtrqJTdi49nnztoEWDJXfDRhyb2fha6PPy");
