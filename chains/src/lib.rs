mod blockchain;
mod consensus;
mod utils;

pub use crate::{
    blockchain::*,
    common::*,
    consensus::*,
    errors::*,
    utils::*,
};

pub mod errors {
    use std::error::Error;

    pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;
}

mod common {
    pub use types::*;

    pub mod types {
        pub type BlockData = String;
        pub type BlockId = u64;
        pub type BlockHash = String;
        pub type BlockNonce = u64;
        pub type BlockTime = bson::DateTime;
    }
}