/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::AsyncSpawable;
use scsys::prelude::{BoxResult, Locked, Logger};

/// Implements the base interface for creating compatible platform applications
pub trait AppSpec: Default + AsyncSpawable {
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
        Ok(self)
    }
}
