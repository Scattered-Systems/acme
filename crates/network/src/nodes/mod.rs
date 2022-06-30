/*
    Appellation: Nodes
    Context: Module
    Creator: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
    Description:

 */
pub use node::*;

mod node;

type NodeError = Box<dyn std::error::Error + Send + Sync + 'static>;

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
    type Config;
    type Data;

    fn constructor(
        &self,
        configuration: Self::Config,
    ) -> Result<Self, NodeError> where Self: Sized;
}