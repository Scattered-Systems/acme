/*
    Appellation: Nodes
    Context: Module
    Creator: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
    Description:

 */
pub use node::*;

mod node;

pub type NodeError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub enum Nodes {
    Full,
    Light,
}

pub enum NodeStates {
    Computing,
    Controlling,
}

pub trait NodeSpec {
    type Account;
    type Client;
    type Configuration;
    type Data;

    fn constructor(
        &self,
        configuration: Self::Configuration,
    ) -> Result<Self, NodeError> where Self: Sized;
}