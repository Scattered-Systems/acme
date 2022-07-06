/*
   Appellation: Nodes
   Context: Module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
        Describes the abstract functionality of nodes and implements a standard interface for
        flexibility
*/
pub use node::*;

mod node;

pub enum NodePartitions {}

/// Outlines the standard interface for creating acme-nodes
pub trait NodeSpec<Addr, Conf, Cont, Data> {
    fn authenticate(&self, context: Cont) -> crate::AuthNoiseKey
        where
            Self: Sized;
    fn configure(&self, configuration: Conf) -> Result<Self, config::ConfigError>
        where
            Self: Sized;
}
