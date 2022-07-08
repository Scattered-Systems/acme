/*
   Appellation: standard
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AppSettings {
    pub mode: String,
    pub name: String,
    pub version: String,
}

impl AppSettings {
    pub fn constructor(mode: String, name: String, version: String) -> Self {
        Self {
            mode,
            name,
            version,
        }
    }
    pub fn new(mode: String, name: String, version: String) -> Self {
        Self::constructor(mode, name, version)
    }
    pub fn from(mode: &str, name: &str, version: &str) -> Self {
        Self::constructor(mode.to_string(), name.to_string(), version.to_string())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum StandardConfigurations {
    Application(AppSettings),
    Database { name: String, uri: String },
    Logger { level: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let a = AppSettings::from("dev", "acme", "0.1.0");
        let app_settings = StandardConfigurations::Application(a.clone());
        assert_eq!(
            app_settings.clone(),
            StandardConfigurations::Application(a.clone())
        )
    }
}
