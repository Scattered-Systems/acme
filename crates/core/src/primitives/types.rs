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