/*
   Appellation: models
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use accounts::*;
pub use assets::*;
pub use portfolios::*;
pub use users::*;
pub use utils::*;

mod accounts;
mod assets;
mod portfolios;
mod users;

/// Outlines a standard data model
pub trait Model<Act, Cnf, Cnt, Dt> {
    fn configure(&self, config: Cnf) -> Result<Self, config::ConfigError>
        where
            Self: Sized;
    fn create(&self, data: Vec<Dt>) -> Result<Self, Box<dyn std::error::Error>>
        where
            Self: Sized;
}

/// Outlines an asynchronous standard data model
pub trait AsyncModel {}

mod utils {
    use std::str::Bytes;

    /// TODO: Finish implementing the hasher with the proper level of cryptography
    pub fn create_hashed_password(password: &str) -> Bytes<'_> {
        let hashed_password = "";
        hashed_password.bytes()
    }
}