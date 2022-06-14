use crate::{models, proofs, schemas, structures};

pub mod constants {
    pub use super::models::constants::*;
    pub use super::proofs::constants::*;
    pub use super::schemas::constants::*;
    pub use super::structures::constants::*;
}

pub mod types {
    use bson;
    use chrono;

    pub use super::models::types::*;
    pub use super::proofs::types::*;
    pub use super::schemas::types::*;
    pub use super::structures::types::*;

    pub type DateTime = chrono::DateTime<LocalTime>;
    pub type LocalTime = chrono::Local;
    pub type TimeStamp = bson::DateTime;

    pub type ObjectId = bson::oid::ObjectId;
}