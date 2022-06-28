/*
    Appellation: Actors
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:

 */
pub use crate::actors::{
    actor::*,
    loggers::*,
    specifications::*,
};

pub(crate) mod loggers;
pub(crate) mod actor;

mod specifications {
    pub trait CLI {
        type Commands;

        fn commands(&self) -> Self::Commands;
    }
}