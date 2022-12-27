/*
    Appellation: cli <specs>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Handler, AsyncHandler};
use scsys::AsyncResult;

///
pub trait CLISpec: clap::Parser {
    type Cmds: Handler + clap::Subcommand;

    fn command(&self) -> Option<Self::Cmds>
    where
        Self: Sized;
    fn handler(&self) -> scsys::Result<&Self>
    where
        Self: Sized,
    {
        if let Some(cmd) = self.command() {
            cmd.handler()?;
        }
        Ok(self)
    }
}
///
#[async_trait::async_trait]
pub trait AsyncCLISpec: clap::Parser {
    type Cmds: AsyncHandler + clap::Subcommand;

    fn command(&self) -> Option<Self::Cmds>
    where
        Self: Sized;
    async fn handler(&self) -> AsyncResult<&Self>
    where
        Self: Sized,
    {
        if let Some(cmd) = self.command().clone() {
            cmd.handler().await?;
        }
        Ok(self)
    }
}