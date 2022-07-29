/*
    Appellation: actors <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use actor::*;
pub use utils::*;

mod actor;

/// Implement an abstract actor
pub trait Act<Cnf, Cnt, Dt> {
    fn access(address: String) -> bool
        where
            Self: Sized;
}

mod utils {}
