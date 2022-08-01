/*
    Appellation: acme <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Acme is designed to simplify the creation of agile, data-centric application within Rust
        leveraging popular frameworks such as axum, clap, and tokio
*/
#[doc(inline)]
pub use crate::{actors::*, components::*, core::*, data::*};
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "macros")]
pub use acme_macros::*;
#[cfg(feature = "network")]
pub use acme_network as network;

mod actors;
mod components;
mod core;
mod data;

pub mod prelude {
    #[cfg(feature = "network")]
    pub use super::network::prelude::*;
    pub use super::{
        clients::*, connect::*, core::*, create::*, databases::*, loggers::*, models::*, update::*,
        verify::*,
    };
}
