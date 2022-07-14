/*
   Appellation: context
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use components::*;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub logger: Logger,
}

impl Settings {
    fn constructor() -> Result<acme::DefaultConfigBuilder, config::ConfigError> {
        let mut builder = config::Config::builder();

        builder = builder.set_default("logger.level", "info")?;

        Ok(builder)
    }
    pub fn new() -> Result<Self, config::ConfigError> {
        Self::constructor().ok().unwrap().build()?.try_deserialize()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Context<Conf = Settings> {
    pub settings: Conf,
    pub timestamp: i64,
}

impl<Conf> Context<Conf> {
    fn constructor(settings: Conf, timestamp: i64) -> Result<Self, acme::StandardError> {
        Ok(Self {
            settings,
            timestamp,
        })
    }
    pub fn new(settings: Conf) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        Self::constructor(settings, timestamp).ok().unwrap()
    }
}

mod components {
    use super::*;

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Logger {
        pub level: String,
    }

    impl Logger {
        fn constructor(level: String) -> Result<Self, acme::StandardError> {
            Ok(Self { level })
        }
        pub fn setup(config: &Settings) {
            if std::env::var_os("RUST_LOG").is_none() {
                let level = config.logger.level.as_str();
                std::env::set_var("RUST_LOG", level);
            }
            tracing_subscriber::fmt::init()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
