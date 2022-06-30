/*
    Appellation: connections
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ConnectionStates {
    Authorized {
        client: String
    }
}

impl ConnectionStates {
    fn authorize(client: String) -> Self { Self::Authorized { client } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let data: &str = "test";
        let mut state = ConnectionStates::authorize(String::from(data));
        assert_eq!(&state, &state)
    }
}