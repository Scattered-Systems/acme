/*
    Appellation: acme-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{primitives::*, specs::*, utils::*};

pub mod events;
pub mod sessions;

pub(crate) mod primitives;
pub(crate) mod utils;

pub(crate) mod specs;

pub trait Tracable {}
