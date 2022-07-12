/*
    Appellation: core
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use common::*;

pub mod configurations;

mod common {
    mod constants {}

    mod types {
        use bson;
        use chrono;
        use std::collections::HashMap;

        pub type LocalTime = chrono::Local;
        pub type Dict<T = Vec<String>> = HashMap<String, T>;


        pub enum Ids {
            Alien(String),
            Objects(bson::oid),
            Standard(u64),
        }
    }

    mod variants {}
}