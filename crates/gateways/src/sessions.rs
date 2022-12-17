/*
    Appellation: sessions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{Id, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Session {
    pub id: Id,
    pub timestamp: Timestamp,
    pub data: Vec<String>,
}
