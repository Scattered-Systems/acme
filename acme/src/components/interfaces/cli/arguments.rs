/*
    Appellation: arguments <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(
clap::ArgEnum, Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum CRUDArgs {
    Create,
    Read,
    Update,
    Delete,
}

impl Default for CRUDArgs {
    fn default() -> Self {
        Self::Read
    }
}

#[derive(
clap::ArgEnum, Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum ActionArgs {
    Compute,
    Connect,
    Determine,
    Terminate,
}

impl Default for ActionArgs {
    fn default() -> Self {
        Self::Connect
    }
}

#[derive(
clap::ArgEnum, Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum PowerArgs {
    Start,
    Stop,
    Transition,
}

impl Default for PowerArgs {
    fn default() -> Self {
        Self::Stop
    }
}
