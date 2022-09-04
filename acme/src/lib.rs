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
        core::{api::*, errors::*, types::*},
        data::*
    };

    #[cfg(feature = "web")]
    pub use axum;
    #[cfg(feature = "web")]
    pub use hyper;
    #[cfg(feature = "web")]
    pub use tokio;
    #[cfg(feature = "extras")]
    pub use tower;
    #[cfg(feature = "extras")]
    pub use tower_http;
    #[cfg(feature = "extras")]
    pub use tracing;
    #[cfg(feature = "extras")]
    pub use tracing_subscriber;


    
    #[cfg(feature = "derive")]
    pub use acme_derive::*;
    #[cfg(feature = "macros")]
    pub use acme_macros::*;
}
