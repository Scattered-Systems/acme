/*
    Appellation: state <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::{BsonOid, Timestamp};
use std::string::ToString;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumString,
    strum::EnumVariantNames,
)]
#[strum(serialize_all = "title_case")]
pub enum Rails {
    Capture,
    Compute,
    Connect,
    Control,
    Contribute,
    Extract,
    Null,
}

impl Default for Rails {
    fn default() -> Self {
        Self::Null
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State<T = String> {
    pub id: BsonOid,
    pub data: Vec<T>,
    pub message: Option<String>,
    pub timestamp: Timestamp,
}

impl State {
    pub fn new(message: Option<String>) -> Self {
        let id = BsonOid::new();
        let timestamp = Timestamp::default();
        let data = Vec::new();
        Self {
            id,
            timestamp,
            data,
            message,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{Rails, State};

    #[test]
    fn test_default_rails() {
        let actual = Rails::default();
        let expected = Rails::try_from("Null").expect("Error");

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_default_state() {
        let actual = State::default();
        let expected = State::new(None);

        assert_eq!(actual.data, expected.data)
    }
}
