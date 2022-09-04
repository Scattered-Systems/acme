/*
    Appellation: acme <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Acme is designed to simplify the creation of agile, data-centric application within Rust
        leveraging popular frameworks such as axum, clap, and tokio
*/
#[doc(inline)]

pub mod actors;
pub mod components;
pub mod core;
pub mod data;

#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "macros")]
pub use acme_macros::*;
#[cfg(feature = "network")]
pub use acme_network as network;

pub mod prelude {
    #[cfg(feature = "network")]
    pub use super::network::prelude::*;
    #[cfg(feature = "derive")]
    pub use acme_derive::*;
    #[cfg(feature = "macros")]
    pub use acme_macros::*;
}
