use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn setup() -> Self {
        todo!()
    }
}