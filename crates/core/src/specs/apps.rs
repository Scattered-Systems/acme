/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::AsyncSpawable;
use scsys::prelude::{Locked, Logger, BoxResult};

/// Implements the base interface for creating compatible platform applications
pub trait AppSpec: Default {
type Cnf;
type Ctx;
type State;
fn init() -> Self;
fn context(&self) -> Self::Ctx;
fn name(&self) -> String;
fn settings(&self) -> Self::Cnf;
fn setup(&mut self) -> BoxResult<&Self>;
fn slug(&self) -> String {
    self.name().to_ascii_lowercase()
}
fn state(&self) -> &Locked<Self::State>;
}

/// Extends the core interface to include logging capabilities
pub trait ApplicationLoggerSpec: AppSpec {
    /// Creates a service handle for toggling the tracing systems implemented
    fn with_tracing(&self, level: Option<&str>) -> BoxResult<&Self> {
        // TODO: Introduce a more refined system of tracing logged events
        let mut logger = Logger::new(level.unwrap_or("info").to_string());
        logger.setup(None);
        tracing_subscriber::fmt::init();

        tracing::info!("Successfully initiated the tracing protocol...");
        Ok(self)
    }
}

#[async_trait::async_trait]
pub trait AsyncApplicationSpawner: AsyncSpawable {
    /// Signals a graceful shutdown using tokio channels
    async fn graceful_shutdown(&self) {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to terminate the runtime...");
        tracing::info!("Terminating the application and connected services...");
    }
}

#[cfg(feature = "cli")]
#[cfg(not(feature = "wasm"))]
pub trait CommandLineInterface: clap::Parser {}
