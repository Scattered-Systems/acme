/*
    Appellation: utils
    Context: mod
    Creator:
    Description:
        Core feature library for acme, an all-in-one blockchain toolkit for building optimized
        EVM compatible apps and chains.
 */
mod hashing;
mod validators;

pub use crate::utils::{hashing::*, validators::*};

pub fn timestamp() -> bson::DateTime {
    chrono::Local::now().into()
}