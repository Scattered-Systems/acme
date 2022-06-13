pub mod blockchain;
pub mod utils;

pub mod constants {
    
}

pub mod types {
    use bson;
    use chrono;

    pub type BoxedError = Box<dyn std::error::Error>;

    pub type DateTime = chrono::DateTime<LocalTime>;
    pub type LocalTime = chrono::Local;
    pub type ObjectId = bson::oid::ObjectId;
    pub type TimeStamp = bson::DateTime;
}