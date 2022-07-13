/*
   Appellation: standard
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum StandardConfigurations {
    Application(crate::AppParams),
    Database { name: String, uri: String },
    Logger { level: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let a = crate::AppParams::from("dev", "acme");
        let app_settings = StandardConfigurations::Application(a.clone());
        assert_eq!(
            app_settings.clone(),
            StandardConfigurations::Application(a.clone())
        )
    }
}
