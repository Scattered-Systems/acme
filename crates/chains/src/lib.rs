mod accounts;
mod blockchain;
mod consensus;
mod utils;

pub use crate::{accounts::*, blockchain::*, common::*, consensus::*, errors::*, utils::*};

mod errors {
    use std::error::Error;

    pub type ChainError = Box<dyn Error + Send + Sync + 'static>;
}

mod common {
    pub use constants::*;
    pub use types::*;

    pub mod constants {
        pub const DIFFICULTY_PREFIX: &str = "00";
    }

    pub mod types {
        pub type BlockData = String;
        pub type BlockId = u64;
        pub type BlockHash = String;
        pub type BlockNonce = u64;
        pub type BlockTime = i64;
    }
}
