/*
    Appellation: standard
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */


#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum StandardConfigurations {
    Application {
        mode: String,
        name: String,
    },
    Database {
        name: String,
        uri: String,
    },
    Logger {
        level: String
    },
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let app = StandardConfigurations::Application { mode: "".to_string(), name: "".to_string() };
    }
}