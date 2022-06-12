pub mod blocks;
pub mod chains;
pub mod utils;

pub mod constants {
    pub const DIFFICULTY_PREFIX: &str = "00";
}

pub mod types {
    use bson;
    use chrono;

    pub type BlockData = String;
    pub type BlockId = bson::oid::ObjectId;
    pub type BlockHash = String;
    pub type BlockNonce = u64;

    pub type DateTime = chrono::DateTime<LocalTime>;
    pub type LocalTime = chrono::Local;
    pub type TimeStamp = bson::DateTime;
}