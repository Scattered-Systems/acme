/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
pub use chain::*;

mod chain;

pub enum ChainStates {
    Appending,
    Computing,
    Connecting,
    Determining,
}


pub trait ChainSpec {
    type Appellation;
    // Define the Chain's name
    type Configuration;
    // Type of configuration for the Chain
    type Context;
    // Define the
    type Data; // Define the standard data structure to be use throughout the Chain

    fn activate(appellation: Self::Appellation, configuration: Self::Configuration) -> Self;
    fn chain(&self) -> Result<(), crate::errors::BoxedError>;
    fn constructor(&self) -> Self;
}