/*
   Appellation: options
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
#[derive(clap::Subcommand, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Subcommands {
    Block {
        #[clap(long, required = false, short, parse(from_occurrences))]
        generate: i8,
        #[clap(default_value = "", long, short, value_parser)]
        data: String,
    },
    Chain {
        #[clap(long, required = false, short, parse(from_occurrences))]
        scaffold: i8,
    },
    Cluster {
        #[clap(
            default_value = "",
            forbid_empty_values = false,
            long,
            short,
            value_parser
        )]
        secret: String,
    },
    Wallet {
        #[clap(default_value = "", long, short, value_parser)]
        account: String,
    },
}

#[derive(clap::Args, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataArgs {
    #[clap(default_value = "key", long, required = false, short, value_parser)]
    key: String,
}

#[derive(clap::Parser, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[clap(about, version)]
pub struct Opts<S: clap::Subcommand = Subcommands> {
    #[clap(subcommand)]
    pub context: S,
}
