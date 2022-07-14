/*
   Appellation: components
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use database::*;
pub use logger::*;

mod database;
mod logger;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AppStates<T = String> {
    Initiate(T),
    Start(T),
    Terminate(T),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Configuration {
    pub mode: String,
    pub name: String,
}

impl Configuration {
    pub fn create() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder();

        builder = builder
            .set_default("mode", "production")?
            .set_default("name", "acme")?
            .set_default("logger.name", "acme")?;

        builder.build()?.try_deserialize()
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Settings")
    }
}
