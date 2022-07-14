/*
    Appellation: controls
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(clap::Subcommand, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Context {
    Search {
        #[clap(long, required = false, value_parser)]
        query: String
    }
}

#[derive(clap::ArgEnum, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Control {
    Account,
    Compute,
    Create,
    Discover,
}

#[derive(clap::Parser, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CommandCenter {
    #[clap(arg_enum)]
    pub control: Control,
    #[clap(subcommand)]
    pub context: Context,
}