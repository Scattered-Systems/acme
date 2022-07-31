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
    use crate::async_trait;

    /// Describe a standard api specification to implement on compatible structures
    #[async_trait]
    pub trait APISpec<App> {
        async fn run(&self) -> Result<App, scsys::BoxError>
            where
                Self: Sized;
    }

    /// Describe a standard cli specification to implement on compatible structures
    pub trait CLISpec<App: clap::Parser> {
        fn run(&self) -> Result<App, scsys::BoxError>
            where
                Self: Sized;
    }
}
