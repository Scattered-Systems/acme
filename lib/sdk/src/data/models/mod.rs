/*
   Appellation: models
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use accounts::*;
pub use assets::*;
pub use utils::*;

mod accounts;
mod assets;

pub trait Model<Act, Cnf, Cnt, Dt> {
    fn constructor(&self, config: Cnf) -> Result<Self, config::Error>
        where
            Self: Sized;
}

pub trait AsyncModel {}

mod utils {}
