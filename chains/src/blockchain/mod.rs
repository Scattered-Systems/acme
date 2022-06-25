/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
pub use chain::*;

mod chain;

pub type ChainError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub enum ChainStates {
    Appending,
    Computing,
    Connecting,
    Determining,
}


pub trait ChainSpec {
    type Actor;
    type Client;
    type Config;
    type Data;

    fn constructor(&self, pattern: String) -> Result<Self, ChainError> where Self: Sized;
}