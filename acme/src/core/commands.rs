/*
    Appellation: commands
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use clap::{
    Parser,
    Subcommand,
};

#[derive(Copy, Clone, Debug, PartialEq, Subcommand)]
pub enum Actions {
    Create,
    Read,
    Update,
}

#[derive(Copy, Clone, Debug, PartialEq, Subcommand)]
pub enum Computations {
    Password
}

#[derive(Clone, Debug, Parser)]
#[clap(author = "FL03 <jo3mccain@icloud.com>", version = "0.1.28")]
pub struct Commands {
    #[clap(subcommand)]
    pub actions: Actions,
    #[clap(long)]
    pub client: String,
    #[clap(default_value = "private", long, short, value_parser)]
    pub context: String,
}