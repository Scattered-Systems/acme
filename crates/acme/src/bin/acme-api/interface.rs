/*
   Appellation: interface
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::AsyncStdError;

#[async_trait::async_trait]
pub trait API {
    type Config;
    type Context;
    type Data;
    async fn configure(config: Self::Config) -> Result<Self::Config, config::ConfigError>;
    async fn new() -> Result<Self, AsyncStdError>
    where
        Self: Sized;
}
