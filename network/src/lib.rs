/*
    Appellation: acme-network <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Acme is designed to simplify the creation of agile, data-centric application within Rust
        leveraging popular
*/
#[doc(inline)]
pub use crate::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

pub mod prelude {
    pub use super::{
        actors::*,
        components::{proxies::*, routers::*, servers::*},
        core::*,
        data::*,
    };
}
