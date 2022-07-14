/*
   Appellation: settings
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AppStates<T = String> {
    Initiate(T),
    Start(T),
    Terminate(T),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Settings {
    Application { mode: String, name: String },
    Logger { level: String },
}

impl Settings {
    pub fn create() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder();

        builder = builder
            .set_default("application.mode", "production")?
            .set_default("application.name", "acme")?
            .set_default("logger.name", "acme")?;

        builder.build()?.try_deserialize()
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Settings")
    }
}
