/*
    Appellation: types
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
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