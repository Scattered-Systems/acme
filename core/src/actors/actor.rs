/*
    Appellation: actor
    Context: module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

pub trait ActorSpec {
    type Actor;
    type Config;
    type Context;
    type Data;

    fn actor() -> Self::Actor;
    fn configure(&self) -> Result<Self::Config, config::ConfigError>;
    fn context(&self) -> Self::Context;
    fn data(&self) -> Self::Data;
}