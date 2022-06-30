/*
   Appellation: utils
   Context: mod
   Creator:
   Description:
       Core feature library for acme, an all-in-one blockchain toolkit for building optimized
       EVM compatible apps and chains.
*/
mod hashing;
mod validators;

pub use crate::utils::{hashing::*, validators::*};

#[derive(Clone, Debug, Hash, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct Timestamp;

impl Timestamp {
    pub fn local() -> crate::BlockTime {
        chrono::Local::now().timestamp()
    }
    pub fn utc() -> crate::BlockTime {
        chrono::Utc::now().timestamp()
    }
}

pub fn block_ts_utc() -> crate::BlockTime {
    let ts = chrono::Utc::now();
    ts.timestamp()
}

pub fn block_ts_local() -> crate::BlockTime {
    let ts = chrono::Local::now();
    ts.timestamp()
}
