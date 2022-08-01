/*
    Appellation: mod <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use arguments::*;

pub(crate) mod arguments;

/// Describe a standard cli specification to implement on compatible structures
pub trait CLISpec<App: clap::Parser> {
    fn build() -> Result<App, scsys::BoxError> {
        Ok(App::parse())
    }
    fn run(&self) -> Result<(), scsys::BoxError>
        where
            Self: Sized;
}
