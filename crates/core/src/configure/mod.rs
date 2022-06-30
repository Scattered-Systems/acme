/*
   Appellation: mod
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use configuration::*;
pub use cnf::*;

mod configuration;

mod cnf {
    pub use config::ConfigError;

    pub enum Configurations {
        Standard {
            logger: crate::Logger,
        }
    }

    pub trait Configuration {
        type Data;

        fn constructor() -> config::ConfigBuilder<config::builder::DefaultState>;
        fn new() -> Result<Self, ConfigError> where Self: Sized;
        fn from(data: Self::Data) -> Result<Self, ConfigError> where Self: Sized;
    }
}