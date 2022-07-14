/*
   Appellation: actors
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use aggregators::*;
pub use utils::*;
pub use validators::*;

mod aggregators;
pub mod interfaces;
mod validators;

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
