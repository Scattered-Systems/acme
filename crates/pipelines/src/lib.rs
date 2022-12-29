/*
    Appellation: pzzld-pipelines <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{pipeline::*, primitives::*, stages::*, utils::*};

pub(crate) mod pipeline;
pub(crate) mod stages;
pub(crate) mod utils;

pub(crate) mod primitives {
    pub const DEFAULT_WORKDIR: &str = ".";
}
