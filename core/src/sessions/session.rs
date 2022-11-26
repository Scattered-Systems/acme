/*
    Appellation: session <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

use scsys::prelude::{Event, Timestamp};
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Session {
    pub events: Vec<Event>,
    pub timestamp: i64,
}

impl Session {
    pub fn new(events: Vec<Event>) -> Self {
        let timestamp = Timestamp::default().into();
        Self { events, timestamp }
    }
}

impl From<&Session> for Session {
    fn from(data: &Session) -> Self {
        data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_session() {
        let a = Session::default();
        let b = Session::from(&a);
        assert_eq!(&a, &b)
    }
}
