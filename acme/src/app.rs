/*
    Appellation: app
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use clap::Parser;
use serde::{Deserialize, Serialize};

pub trait CLI {
    type Args;
    fn constructor(&self) -> Self::Args;
}

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


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Application {
    pub appellation: Appellation,
}

impl Application {
    pub fn new(appellation: Appellation) -> Self {
        Self {
            appellation
        }
    }
}

impl CLI for Application {
    type Args = crate::Commands;

    fn constructor(&self) -> Self::Args {
        return Self::Args::parse();
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application(appellation={})", self.appellation)
    }
}