/*
    Appellation: configuration
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Application {
    pub mode: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Config {
    pub application: Application,
    pub logger: Logger,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let patterns = vec!["**/*.config.*"];
        config_builder(&patterns)
    }
}

pub fn config_builder(patterns: &Vec<&str>) -> Result<Config, config::ConfigError> {
    let mut builder = config::Config::builder();

    for p in 0..patterns.len() {
        builder = builder.add_source(
            glob::glob(&*patterns[p])
                .unwrap()
                .map(|path| config::File::from(path.unwrap()).required(false))
                .collect::<Vec<_>>(),
        );
    }

    builder = builder.add_source(config::Environment::default().separator("__"));
    builder.build()?.try_deserialize()
}