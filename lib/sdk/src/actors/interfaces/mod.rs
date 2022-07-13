/*
   Appellation: mod
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use interface::*;

mod interface;

pub trait Interface<Act, Cnf, Cnt, Dt> {
    fn authenticate(&self, actor: Act) -> Self
        where
            Self: Sized;
    fn constructor(&self, data: Vec<Dt>) -> Result<Self, config::ConfigError>
        where
            Self: Sized;
}
