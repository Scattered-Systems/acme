/*
   Appellation: mod
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use utils::*;

pub mod interfaces;

pub trait ActorSpec<Cnf, Data> {
    fn configure(&self, config: Cnf) -> Self
        where
            Self: Sized;
    fn constructor(&self, data: Vec<Data>) -> Self
        where
            Self: Sized;
}

pub struct Actor {
    pub id: crate::Ids,
}

mod utils {}
