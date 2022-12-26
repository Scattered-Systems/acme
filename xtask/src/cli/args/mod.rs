/*
    Appellation: args <cli>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{auto::*, builder::*, runner::*, setup::*};

pub(crate) mod builder;
pub(crate) mod runner;
pub(crate) mod setup;

pub(crate) mod auto {
    use crate::command;
    use anyhow::Result;
    use clap::Args;
    use serde::{Deserialize, Serialize};

    #[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct Auto {
        #[arg(action = clap::ArgAction::SetFalse, long, short)]
        clippy: bool,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        bench: bool,
        #[arg(action = clap::ArgAction::SetFalse, long, short)]
        rustfmt: bool,
        #[arg(action = clap::ArgAction::SetFalse, long, short)]
        test: bool,
    }

    impl Auto {
        fn clippy(&self) -> Result<&Self> {
            tracing::info!("Analyzing the codespace...");
            command("cargo", vec!["clippy", "--all", "--allow-dirty", "--fix"])?;
            Ok(self)
        }
        fn rustfmt(&self) -> Result<&Self> {
            tracing::info!("Formatting the codespace...");
            command("cargo", vec!["fmt", "--all"])?;
            Ok(self)
        }
        fn testing(&self) -> Result<&Self> {
            tracing::info!("Testing the workspace...");

            command(
                "cargo",
                vec!["test", "--all", "--all-features", "--release"],
            )?;
            Ok(self)
        }
        pub fn handler(&self) -> Result<&Self> {
            self.clippy()?.rustfmt()?;            
            super::Builder::default().handler()?;
            self.testing()?;
            Ok(self)
        }
    }
}
