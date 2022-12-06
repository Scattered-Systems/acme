/*
    Appellation: acme-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[doc(inline)]
pub use self::{primitives::*, specs::*};

pub(crate) mod primitives;
pub(crate) mod specs;


pub mod sessions;
