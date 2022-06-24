/*
    Appellation: acme-cli
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Initial efforts are being placed towards designing a project management suite of utilities
        for building scalable, user-centric dApps.
 */

use acme;

use clap::Parser;
use serde::{Deserialize, Serialize};

pub trait CLI {
    type Args;

    fn commands(&self) -> Self::Args;
    fn constructor(&self) -> Self;
}

#[derive(Clone, Debug, Parser)]
pub struct Commands {
    #[clap(default_value = "sample.eth", long, short, value_parser)]
    pub appellation: String,

    #[clap(default_value = "none", long, value_parser)]
    pub compute: String,

    #[clap(default_value = "private", long, short, value_parser)]
    pub context: String,

    #[clap(long, short, value_parser)]
    pub data: String

}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Application {
    pub appellation: String,
}

impl Application {
    pub fn new(appellation: String) -> Self {
        Self {
            appellation
        }
    }
}

impl CLI for Application {
    type Args = Commands;

    fn commands(&self) -> Self::Args {
        return Self::Args::parse();
    }

    fn constructor(&self) -> Self {
        todo!()
    }
}


fn main() {
    
    let app = Application::new(String::from("acme"));
    let args = app.commands();

    println!("Welcome to acme {}", args.appellation);
}