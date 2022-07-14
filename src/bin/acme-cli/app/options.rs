/*
   Appellation: options
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(clap::ArgEnum, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BaseArgs {
    Generate,
    Manage,
    Update,
}

#[derive(clap::Subcommand, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Subcommands {
    Account {
        #[clap(long, short, required = false, parse(from_occurrences))]
        login: i8,
    },
    Store {
        #[clap(flatten)]
        data: DataArgs,
    },
}

#[derive(clap::Args, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataArgs {
    #[clap(default_value = "key", long, required = false, short, value_parser)]
    pub key: String,
    #[clap(default_value = "value", long, required = false, short, value_parser)]
    pub value: String,
}

#[derive(clap::Parser, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[clap(about, version)]
pub struct Opts<S: clap::Subcommand = Subcommands> {
    #[clap(arg_enum)]
    pub args: BaseArgs,
    #[clap(subcommand)]
    pub context: S,
}
