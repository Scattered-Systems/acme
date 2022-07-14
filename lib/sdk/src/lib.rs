/*
    Appellation: acme-sdk
    Context: library
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Acme is a complete client library for building data-centric Rust applications

*/
#[doc(inline)]
pub use crate::{actors::*, core::*, data::*};

#[doc(inline)]
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use acme_macros::*;

mod actors;
mod core;
mod data;
