/*
    Appellation: acme
    Context: library
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Acme is a complete client library for building data-centric Rust applications

*/
#[doc(inline)]
#[cfg(feature = "default")]
pub use crate::{actors::*, clients::*, core::*, data::*};

#[doc(inline)]
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use acme_macros::*;


mod actors;
mod clients;
mod core;
mod data;