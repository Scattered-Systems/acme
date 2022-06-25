/*
    Appellation: application
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use clap::Parser;
use config::ConfigError;
use serde::{Deserialize, Serialize};

use crate::{Appellation, CLI};

pub struct AppData {
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Application {
    pub appellation: Appellation
}

impl Application {
    pub fn new(name: &str) -> Self {
        Self {
            appellation: Appellation::from(name)
        }
    }
}

impl CLI for Application {
    type Args = crate::Commands;
    type Config = crate::Configuration;

    fn configure(&self) -> Result<Self::Config, config::ConfigError> {
        let pattern = "**/*.config.*";
        Self::Config::new(pattern)
    }

    fn constructor(&self) -> Self::Args {
        return Self::Args::parse();
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application(appellation={})", self.appellation)
    }
}