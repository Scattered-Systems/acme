/*
   Appellation: acme-core
   Crafter:
   Description:
*/
pub use crate::{common::*, models::*, proofs::*, schemas::*, structures::*};

pub(crate) mod models;
pub(crate) mod proofs;
pub(crate) mod schemas;
pub(crate) mod structures;

mod common {
    pub use constants::*;
    pub use types::*;

    mod constants {}

    mod types {
        pub use bson::DateTime as Timestamp;
        pub use chrono::Local as Localtime;

        pub type AccessGrant = [String; 12];
        pub type Container<T = Vec<String>> = std::collections::HashMap<String, T>;
        pub type ContentId = bson::oid::ObjectId;
    }
}
