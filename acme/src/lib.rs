/*
    Appellation: acme
    Context: library
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Acme is an all-in-one developer tool for blockchain applications, contracts, and sidechains.

*/
#[doc(inline)]
#[cfg(feature = "default")]
pub use acme_sdk::*;

pub use crate::{configurations::*, interfaces::*};

mod configurations;
mod interfaces;
