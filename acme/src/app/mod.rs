/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
pub use crate::app::{
    application::*,
    configuration::*,
};

mod application;
mod configuration;

pub trait CLI {
    type Args;
    type Config;

    fn configure(&self) -> Result<Self::Config, config::ConfigError>;
    fn constructor(&self) -> Self::Args;
}