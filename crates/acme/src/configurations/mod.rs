/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use crate::configurations::{standard::*, utils::*};

mod standard;

pub trait Configurable {
    type Config;

    fn configure(config: Self::Config) -> Result<Self::Config, config::ConfigError>;
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AppSettings {
    pub mode: String,
    pub name: String,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LoggerSettings {
    pub level: String,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    application: AppSettings,
    logger: LoggerSettings,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut builder = construct_config_builder();
        builder = builder
            .set_default("application.mode", "dev")?
            .set_default("application.name", "acme")?
            .set_default("logger.level", "info")?;
        builder = collect_config_files(builder, "**/*.config.*", false);

        builder.build()?.try_deserialize()
    }
}

impl std::fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application(mode={}, name={})", self.mode, self.name)
    }
}

impl std::fmt::Display for LoggerSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logger(level={})", self.level)
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Settings(application={}, logger={})", self.application, self.logger)
    }
}


mod utils {
    use config::{ConfigBuilder, builder::DefaultState};

    pub type ConfigBuilderDS = ConfigBuilder<DefaultState>;

    pub fn construct_config_builder() -> ConfigBuilderDS {
        let mut builder = config::Config::builder();
        builder
    }

    pub fn collect_config_files(builder: ConfigBuilderDS, pattern: &str, required: bool) -> ConfigBuilderDS {
        builder.add_source(
            glob::glob(pattern)
                .unwrap()
                .map(|path| config::File::from(path.unwrap()).required(required))
                .collect::<Vec<_>>()
        )
    }
}