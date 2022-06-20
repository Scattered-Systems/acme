use clap::Parser;
use serde::{Deserialize, Serialize};

pub trait CLI {
    type Args;

    fn commands(&self) -> Self::Args;
    fn constructor(&self) -> Self;
}

#[derive(Clone, Debug, Parser)]
pub struct Commands {
    #[clap(short, long, value_parser, default_value = "app")]
    pub appellation: String,

    #[clap(short, long, value_parser, default_value_t = 1)]
    pub count: u8,

    #[clap(long, short, value_parser, default_value = "myc")]
    pub scaffold: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application;

impl CLI for Application {
    type Args = Commands;

    fn commands(&self) -> Self::Args {
        return Self::Args::parse();
    }

    fn constructor(&self) -> Self {
        todo!()
    }
}