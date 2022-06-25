pub use bson::{DateTime as Timestamp, oid::ObjectId as BlockId};

pub use crate::{
    blockchain::*,
    blocks::*,
};

pub mod blockchain;
pub mod blocks;
pub mod consensus;
pub mod contracts;
pub mod utils;

pub mod errors {
    use std::error::Error;

    pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;
}