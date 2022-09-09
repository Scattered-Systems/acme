/*
    Appellation: acme <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Acme is designed to simplify the creation of agile, data-centric application within Rust
        leveraging popular frameworks such as axum, clap, and tokio
*/
#[doc(inline)]
pub use self::{actors::*, components::*, core::*, data::*};

mod actors;
mod components;
mod core;
mod data;

#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "macros")]
pub use acme_macros::*;

pub mod prelude {
    pub use crate::{
        actors::handlers::*,
        components::{proxies::*, routers::*, servers::*},
        core::api::*,
        data::*,
    };
    #[cfg(feature = "api")]
    pub use axum;
    #[cfg(feature = "web")]
    pub use http;
    #[cfg(feature = "api")]
    pub use hyper;
    #[cfg(feature = "web")]
    pub use reqwest;
    #[cfg(feature = "api")]
    pub use tokio;
    #[cfg(feature = "api")]
    pub use tokio_stream;
    #[cfg(feature = "api")]
    pub use tower;
    #[cfg(feature = "api")]
    pub use tower_cookies;
    #[cfg(feature = "api")]
    pub use tower_http;
    #[cfg(feature = "api")]
    pub use tracing;
    #[cfg(feature = "api")]
    pub use tracing_subscriber;

    #[cfg(feature = "derive")]
    pub use acme_derive::*;
    #[cfg(feature = "macros")]
    pub use acme_macros::*;
}
