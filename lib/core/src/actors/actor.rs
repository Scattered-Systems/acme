/*
    Appellation: actor
    Context: module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

pub trait Actor {
    type Cache;
    type Config;
    type Data;

    fn aggregate(&self, data: Self::Data) -> Self::Cache;
    fn configure(&self, config: Self::Config) -> Result<Self::Config, config::ConfigError>;
    fn control(&self, data: Self::Data) -> Self;
}

