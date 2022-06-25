/*
    Appellation: appellation
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Appellation {
    pub name: String,
    pub slug: String,
}

impl Appellation {
    pub fn new(name: String) -> Self {
        Self {
            name: name.clone(),
            slug: name.to_lowercase().clone(),
        }
    }

    pub fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            slug: String::from(name).to_lowercase(),
        }
    }
}

impl std::fmt::Display for Appellation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Appellation(name={}, slug={})", self.name, self.slug)
    }
}