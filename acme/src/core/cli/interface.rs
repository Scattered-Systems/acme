/*
    Appellation: interfaces <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum IState {
    Start,
    Stop,
}

impl Default for IState {
    fn default() -> Self {
        Self::Stop
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InterfaceConfiguration {
    pub author: String,
    pub description: String,
    pub license: String,
    pub mode: String,
    pub name: String,
    pub slug: String,
    pub version: String,
}

impl InterfaceConfiguration {
    fn constructor(
        author: String,
        description: String,
        license: String,
        mode: String,
        name: String,
        slug: String,
        version: String,
    ) -> Self {
        Self {
            author,
            description,
            license,
            mode,
            name,
            slug,
            version,
        }
    }
    pub fn new(
        author: String,
        description: String,
        license: String,
        mode: String,
        name: String,
        version: String,
    ) -> Self {
        Self::constructor(
            author,
            description,
            license,
            mode,
            name.clone(),
            name.clone().to_lowercase(),
            version,
        )
    }
}

impl Default for InterfaceConfiguration {
    fn default() -> Self {
        Self::new(
            String::new(),
            String::new(),
            String::new(),
            "development".to_string(),
            "Application".to_string(),
            "0.1.0".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config() {
        let actual = InterfaceConfiguration::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
