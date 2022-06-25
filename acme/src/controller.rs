/*
    Appellation: controller
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use config::ConfigError;
use serde::{Deserialize, Serialize};

pub trait Configurator {
    type Config;

    fn configure(&self) -> Result<Self::Config, config::ConfigError>;
    fn constructor(&self) -> Result<Self, config::ConfigError> where Self: Sized;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Configuration {
    pub application: crate::Application,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Controller {
    pub application: crate::Application,
    pub client: String,
}

impl Configurator for Controller {
    type Config = Configuration;

    fn configure(&self) -> Result<Self::Config, ConfigError> {
        todo!()
    }

    fn constructor(&self) -> Result<Self, config::ConfigError> where Self: Sized {
        let appellation = crate::Appellation::from("Acme");

        let mut builder = config::Config::builder()
            .set_default("application.appellation.name", appellation.name)?
            .set_default("application.appellation.slug", appellation.slug)?;

        builder = builder.add_source(glob::glob("**/*.config.*")
            .unwrap()
            .map(|path| config::File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
        );

        builder = builder.add_source(config::Environment::default().separator("__"));
        builder.build()?.try_deserialize()
    }
}