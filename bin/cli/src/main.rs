/*
    Appellation: acme-cli
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
use acme::CLI;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(clap::Args, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct App {
    #[clap(long, required = false, short, parse(from_occurrences))]
    update: i8,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, clap::Subcommand)]
pub enum Contexts {
    Block {
        #[clap(long, required = false, short, parse(from_occurrences))]
        generate: i8,
        #[clap(default_value = "", long, short, value_parser)]
        data: String,
    },
    Chain {
        #[clap(long, required = false, short, parse(from_occurrences))]
        scaffold: i8
    },
    Cluster {
        #[clap(default_value = "", forbid_empty_values = false, long, short, value_parser)]
        secret: String
    },
    Wallet {
        #[clap(default_value = "", long, short, value_parser)]
        account: String
    },
}


#[derive(Clone, Debug, Deserialize, clap::Parser, PartialEq, Serialize)]
#[clap(about, version)]
pub struct Opts<S: clap::Subcommand = Contexts> {
    #[clap(flatten)]
    pub app: App,

    #[clap(subcommand)]
    pub context: S,

}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Application;

impl Application {
    pub fn configure(&self) -> acme::Config {
        let configuration = match acme::Config::new() {
            Ok(v) => v,
            Err(e) => panic!("Configuration Error: {}", e)
        };
        return configuration.clone()
    }
    pub fn new() -> Self {
        Self
    }
}

impl CLI for Application {
    type Commands = Opts<Contexts>;

    fn commands(&self) -> Self::Commands {
        Self::Commands::parse()
    }
}


fn main() {
    let app = Application::new();
    println!("{:#?}", &app.configure());
    println!("{:#?}", &app.commands())
}

