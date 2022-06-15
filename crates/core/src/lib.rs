/*
    Library: acme-core
    Version: 0.1.5

    Overview

 */
pub mod actors;
pub mod controllers;
pub mod contexts;

pub use constants::*;
pub use types::*;
pub use utils::*;

mod constants {
    pub const DIFFICULTY_PREFIX: &str = "00";
}

mod types {
    use bson;
    use chrono;

    pub type BlockData = String;
    pub type BlockId = ObjectId;
    pub type BlockHash = String;
    pub type BlockNonce = u64;

    pub type BoxedError = Box<dyn std::error::Error>;

    pub type DateTime = chrono::DateTime<LocalTime>;
    pub type LocalTime = chrono::Local;
    pub type ObjectId = bson::oid::ObjectId;
    pub type TimeStamp = bson::DateTime;
}

mod utils {
    use crate::types::{LocalTime, TimeStamp};

    pub fn timestamp() -> TimeStamp {
        let current_time: TimeStamp = LocalTime::now().into();
        return current_time;
    }

}