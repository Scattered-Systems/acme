/*
   Appellation: acme-core
   Crafter:
   Description:
*/
pub mod models;
pub mod proofs;
pub mod schemas;
pub mod structures;

pub use common::*;

mod common {
    pub use constants::*;
    pub use types::*;

    mod constants {}

    mod types {
        pub type Dictionary<T = String> = std::collections::HashMap<String, T>;
    }
}
