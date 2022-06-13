pub mod models;
pub mod proofs;
pub mod schemas;
pub mod structures;

pub mod states {
    pub enum BlockStates {
        Invalid,
        Valid,
        Validating
    }

    pub enum ChainStates {
        Initializing,
        Initialized,
        Terminating,
        Terminated
    }
}

pub mod types {
    use bson;
    use chrono;

    pub type DateTime = chrono::DateTime<LocalTime>;
    pub type LocalTime = chrono::Local;
    pub type TimeStamp = bson::DateTime;

    pub type ObjectId = bson::oid::ObjectId;
}