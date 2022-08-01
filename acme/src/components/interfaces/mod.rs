/*
    Appellation: interfaces <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use cli::*;
pub use interface::*;

pub(crate) mod cli;
pub(crate) mod interface;

/// Outlines an abstract interface for implementing interfaces
pub trait Malleable<Act, Cnf, Cnt> {
    fn context(settings: Cnf) -> Result<Cnt, scsys::BoxError>
        where
            Self: Sized;
}
