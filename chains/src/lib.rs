/*
   Appellation: acme-chains
   Context: library
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Chains are collections of Blocks, hashable objects which may be leveraged in a number of
       unique ways from creating decentralized ledgers to serving as a global state machine
*/
pub use crate::{actors::*, controllers::*, core::*, data::*};

mod actors;
mod controllers;
mod core;
mod data;
