/*
    Appellation: utils
    Context: mod
    Creator:
    Description:
        Core feature library for acme, an all-in-one blockchain toolkit for building optimized
        EVM compatible apps and chains.
 */
pub use blockchain::*;

mod blockchain;

pub fn timestamp() -> bson::DateTime {
    chrono::Local::now().into()
}