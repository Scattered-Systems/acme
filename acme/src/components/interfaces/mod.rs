/*
    Appellation: interfaces <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use application::*;

mod application;

/// Outlines an abstract interface for implementing interfaces
pub trait Malleable<Act, Cnf, Cnt> {
    fn context(settings: Cnf) -> Result<Cnt, scsys::BoxError>
        where
            Self: Sized;
}

pub mod flavors {
    use crate::{async_trait, AxumServer};

    /// Describe a standard api specification to implement on compatible structures
    #[async_trait]
    pub trait APISpec<App = AxumServer> {
        async fn run(&self) -> Result<(), scsys::BoxError>
            where
                Self: Sized + Sync;
    }

    /// Describe a standard cli specification to implement on compatible structures
    pub trait CLISpec<App: clap::Parser> {
        fn build() -> Result<App, scsys::BoxError> {
            Ok(App::parse())
        }
        fn run(&self) -> Result<(), scsys::BoxError>
            where
                Self: Sized;
    }
}
