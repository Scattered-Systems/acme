/*
    Appellation: controls
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(clap::Subcommand, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Context {
    Generate {
        #[clap(default_value_t = 0, long, short, value_parser)]
        count: usize,
        #[clap(default_value = "", long, value_parser)]
        class: String,
    },
    Manage {
        #[clap(default_value = "", long, short, value_parser)]
        address: String,
    },
    Search {
        #[clap(default_value = "", long, required = false, short, value_parser)]
        query: String,
    },
}

#[derive(clap::ArgEnum, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Control {
    Application,
    Capture,
    Credentials,
    Discover,
}

#[derive(clap::Parser, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CommandCenter {
    #[clap(arg_enum)]
    pub control: Control,
    #[clap(subcommand)]
    pub context: Context,
}
