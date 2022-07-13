/*
   Appellation: mod
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use components::*;
pub use standard::*;
pub use utils::*;

mod standard;

pub trait Configurable {
    type Config;

    fn configure(config: Self::Config) -> Result<Self::Config, config::ConfigError>;
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Configuration {
    pub application: AppParams,
    pub database: DatabaseParams,
    pub logger: LoggerParams,
    pub server: ServerParams,
}

impl Configuration {
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

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Settings(application={}, logger={})",
            self.application, self.logger
        )
    }
}

mod components {
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct AppParams {
        pub mode: String,
        pub name: String,
    }

    impl AppParams {
        fn constructor(mode: String, name: String) -> Self {
            Self { mode, name }
        }

        pub fn new(mode: String, name: String) -> Self {
            Self::constructor(mode, name)
        }

        pub fn from(mode: &str, name: &str) -> Self {
            Self::constructor(String::from(mode), String::from(name))
        }
    }

    impl std::fmt::Display for AppParams {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Application(mode={}, name={})", self.mode, self.name)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct DatabaseParams {
        pub name: String,
        pub uri: String,
    }

    impl std::fmt::Display for DatabaseParams {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Database(name={}, uri={})", self.name, self.uri)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct LoggerParams {
        pub level: String,
    }

    impl LoggerParams {
        fn constructor(level: String) -> Self {
            Self { level }
        }
        pub fn setup(config: &crate::Configuration) -> Self {
            if std::env::var_os("RUST_LOG").is_none() {
                let app_name = config.application.name.as_str();
                let level = config.logger.level.as_str();
                let env = format!("api={},tower_http={}", app_name, level);

                std::env::set_var("RUST_LOG", env);
            }

            tracing_subscriber::fmt::init();
            Self::constructor(config.logger.level.clone())
        }
    }

    impl std::fmt::Display for LoggerParams {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Logger(level={})", self.level)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct ServerParams {
        pub port: u16,
        pub host: String,
    }

    impl std::fmt::Display for ServerParams {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Server(port={})", self.port)
        }
    }
}

mod utils {
    use crate::ConfigBuilderDS;

    pub fn construct_config_builder() -> ConfigBuilderDS {
        config::Config::builder()
    }

    pub fn collect_config_files(
        builder: ConfigBuilderDS,
        pattern: &str,
        required: bool,
    ) -> ConfigBuilderDS {
        builder.add_source(
            glob::glob(pattern)
                .unwrap()
                .map(|path| config::File::from(path.unwrap()).required(required))
                .collect::<Vec<_>>(),
        )
    }

    pub fn collect_host_from_string<T>(string: String, breakpoint: char) -> Vec<T>
        where
            T: Clone + std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let exclude: &[char] = &[' ', ',', '[', ']', '.'];
        let trimmed: &str = &string.trim_matches(exclude);
        trimmed
            .split(breakpoint)
            .map(|i| i.trim_matches(exclude).parse::<T>().unwrap())
            .collect()
    }
}
