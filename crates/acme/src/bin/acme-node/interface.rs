/*
    Appellation: interface
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
use clap::Parser;

pub enum Apps {
    Api,
    Cli,
    Gui,
}


#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct App {
    pub development: bool,
    pub name: String,
}

impl App {
    pub fn configure(&self) -> acme::Config {
        let configuration = match acme::Config::new() {
            Ok(v) => v,
            Err(e) => panic!("Configuration Error: {}", e)
        };
        return configuration.clone();
    }

    pub fn new(development: bool, name: String) -> Self {
        Self {
            development,
            name,
        }
    }
}

impl crate::CLI for App {
    type Commands = crate::Opts<crate::Contexts>;

    fn call(&self) -> Self::Commands {
        Self::Commands::parse()
    }
}