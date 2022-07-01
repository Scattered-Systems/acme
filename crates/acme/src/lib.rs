/*
    Appellation: acme
    Context: library
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Acme is the designated application library, strapping a cli with seperate async p2p applications
*/
#[doc(inline)]
#[cfg(feature = "default")]
pub use acme_sdk::*;

pub use crate::{configurations::*, interfaces::*};

mod configurations;
mod interfaces;