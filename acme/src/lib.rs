/*
    Appellation: acme <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        Acme is designed to simplify the creation of agile web applications written in Rust
*/
#[doc(inline)]
pub use self::{actors::*, components::*, core::*, data::*};
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "macros")]
pub use acme_macros::*;

mod actors;
mod components;
mod core;
mod data;

pub mod prelude {
    pub use crate::{
        actors::handlers::*,
        components::{clients::*, proxies::*, routers::*, servers::*},
        core::api::*,
        data::*,
    };
    #[cfg(feature = "core")]
    pub use axum;
}
