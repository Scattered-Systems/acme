/*
    Appellation: Peers
    Context: Module
    Creator: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
    Description:
 */
pub use peer::*;

mod peer;

pub type PeerError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait PeerSpec {
    type Actor;
    type Config;
    type Data;
    fn configure(config: Self::Config) -> Result<Self::Config, config::ConfigError>;
    fn new() -> Self;
}

