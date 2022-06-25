pub mod chats;
pub mod ipfs;
pub mod standard;
pub mod storage;
pub mod utils;

pub use crate::behaviours::{
    chats::*,
    ipfs::*,
    standard::*,
    storage::*,
    utils::*
};