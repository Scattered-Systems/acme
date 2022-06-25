/*
    Appellation: acme-core
    Crafter:
    Description:
 */
pub use crate::{
    common::*,
    models::*,
    proofs::*,
    schemas::*,
    structures::*,
};

pub(crate) mod models;
pub(crate) mod proofs;
pub(crate) mod schemas;
pub(crate) mod structures;

mod common {
    pub use types::*;

    mod types {
        use bson::DateTime as Timestamp;
        use bson::oid::ObjectId;

        pub type AccessGrant = [String; 12];
        pub type Container<T = Vec<String>> = std::collections::HashMap<String, T>;
    }
}