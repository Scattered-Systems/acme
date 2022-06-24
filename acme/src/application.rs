use clap::Parser;
use serde::{Deserialize, Serialize};

pub trait CLI {
    type Args;

    fn commands(&self) -> Self::Args;
    fn constructor(&self) -> Self;
}

#[derive(Clone, Debug, Parser)]
pub struct Commands {
    #[clap(long, short)]
    pub appellation: String

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