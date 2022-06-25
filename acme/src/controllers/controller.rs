/*
    Appellation: controller
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use config::ConfigError;
use serde::{Deserialize, Serialize};

pub trait Configurable {
    type Actor;
    type Context;

    fn actor(&self, name: &str) -> Self::Actor;
    fn context(&self) -> Self::Context;
    fn constructor(&self) -> Result<Self, ConfigError> where Self: Sized;
}

pub trait Contextual {
    type Actor;
    type Config;
    type Data;

    fn constructor(&self) -> Result<Self, acme::BoxedError> where Self: Sized;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Configuration {
    pub application: crate::Application,
}

impl Configuration {
    pub fn new(pattern: &str) -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .set_default("application.appellation.name", "Acme")?
            .set_default("application.appellation.slug", "acme")?;

        builder = builder.add_source(glob::glob(pattern)
            .unwrap()
            .map(|path| config::File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
        );

        builder = builder.add_source(config::Environment::default().separator("__"));
        builder.build()?.try_deserialize()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Context {
    pub settings: Configuration,
}

impl Context {
    pub fn new(settings: Configuration) -> Self {
        Self {
            settings
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Controller {
    pub application: crate::Application,
}

impl Configurable for Controller {
    type Actor = Self;
    type Context = Context;

    fn actor(&self, name: &str) -> Self::Actor {
        Self::Actor {
            application: crate::Application::new(name)
        }
    }

    fn context(&self) -> Self::Context {
        let settings = match Configuration::new("**/*.config.*") {
            Ok(val) => { val }
            Err(err) => { panic!("Configuration Error: {}", err) }
        };
        Self::Context {
            settings
        }
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