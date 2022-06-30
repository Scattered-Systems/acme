/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
mod endpoints;

pub use endpoints::*;

type APIResultError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[async_trait::async_trait]
pub trait API {
    type Config;
    type Context;
    type Data;
    async fn configure(config: Self::Config) -> Result<Self::Config, config::ConfigError>;
    async fn new() -> Result<Self, APIResultError> where Self: Sized;
}