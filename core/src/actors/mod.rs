/*
   Appellation: actors
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:

*/
pub use actor::*;
pub use interfaces::*;

mod actor;
mod interfaces;

pub enum Actions {
    Authorize,
    Append,
    Create,
    Read,
    Update,
    Delete,
    Compile,
    Compute,
    Consent,
    Transact,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum ActionStatus<T = String> {
    Acting(T),
    Completed(T),
    Exited(T),
}

pub trait Actionable {
    type Action;
    type Config;
    type Context;
    type Data;

    fn constructor(action: Self::Action, config: Self::Config, data: Self::Data) -> Self;
    fn determine(&self) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>
    where
        Self: Sized;
}