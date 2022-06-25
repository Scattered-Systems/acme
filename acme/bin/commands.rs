/*
    Appellation: commands
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Commands {
    #[clap(default_value = "sample.eth", long, short, value_parser)]
    pub appellation: String,

    #[clap(default_value = "none", long, value_parser)]
    pub compute: String,

    #[clap(default_value = "private", long, short, value_parser)]
    pub context: String,

    #[clap(long, short, value_parser)]
    pub data: String,
}