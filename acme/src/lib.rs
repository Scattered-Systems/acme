/*
    Appellation: acme <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        Acme is designed to simplify the creation of agile web applications written in Rust
*/
#[doc(inline)]
#[cfg(feature = "core")]
pub use acme_core as core;
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "macros")]
pub use acme_macros::*;

pub mod prelude {}
