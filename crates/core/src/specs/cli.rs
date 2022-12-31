/*
    Appellation: cli <specs>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Handler, AsyncHandler};
use scsys::AsyncResult;

///
pub trait CLISpec: clap::Parser + Handler {
    type Cmds: Handler + clap::Subcommand;
    fn new() -> Self {
        Self::parse()
    }
    fn command(&self) -> Option<Self::Cmds>
    where
        Self: Sized;
    fn handler(&self) -> scsys::Result<&Self>
    where
        Self: Sized;
}
///
#[async_trait::async_trait]
pub trait AsyncCLISpec: clap::Parser + AsyncHandler {
    type Cmds: AsyncHandler + clap::Subcommand;
    fn new() -> Self {
        Self::parse()
    }
    fn command(&self) -> Option<Self::Cmds>
    where
        Self: Sized;
    async fn handler(&self) -> AsyncResult<&Self>
    where
        Self: Sized;
}