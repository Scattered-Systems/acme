/*
    Appellation: acme-core
    Context: library
    Creator:
    Description:
        Core feature library for acme, an all-in-one blockchain toolkit for building optimized
        EVM compatible apps and chains.
 */
mod actors;
mod contexts;
mod controllers;
mod core;
mod errors;
mod utils;

pub use crate::{
    actors::*,
    common::*,
    controllers::*,
    core::*,
    errors::*,
    utils::*,
};

mod common {
    pub use bson::{DateTime as TimeStamp, oid::ObjectId};
    pub use chrono::Local as LocalTime;

    pub type BlockData = String;
    pub type BlockId = ObjectId;
    pub type BlockHash = String;
    pub type BlockNonce = u64;

    pub type Container<T = String> = Dictionary<Vec<T>>;
    pub type Dictionary<T = String> = std::collections::HashMap<String, T>;

    pub enum Dates {
        Datetime(chrono::DateTime<chrono::Local>),
        Localtime(chrono::Local),
        Timestamp(bson::DateTime),
    }

    pub type DateTime = chrono::DateTime<LocalTime>;
}