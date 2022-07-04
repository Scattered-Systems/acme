/*
   Appellation: mod
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use pos::*;
pub use pow::*;

mod pos;
mod pow;

pub enum ConsensusMechanisms {
    PoS,
    PoW,
}
