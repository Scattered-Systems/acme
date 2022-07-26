/*
    Appellation: client <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ClientState {
    Created,
    Connected,
    Connecting,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Client {
    pub name: String,
    pub slug: String,
    pub state: ClientState,
}

impl Client {
    fn constructor(name: String, slug: String, state: ClientState) -> Self {
        Self { name, slug, state }
    }

    pub fn new(name: String, state: ClientState) -> Self {
        Self::constructor(name.clone(), name.to_lowercase().clone(), state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client() {
        let name = "acme".to_string();
        let actual = Client::new(name, ClientState::Connecting);
        assert_eq!(&actual, &actual)
    }
}
