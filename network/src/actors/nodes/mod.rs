/*
    Appellation: Nodes
    Context: Module
    Creator: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
    Description:

 */
mod node;
pub use node::*;

pub enum Nodes {
    Full,
    Light
}

pub enum NodeStates {
    Computing,
    Controlling
}

pub trait NodeSpec {
    type Appellation;
    type Client;
    type Configuration;
    type Data;

    fn activate(appellation: Self::Appellation) -> Self;
    fn configure(configuration: Self::Configuration) -> Self;
    fn connect(&mut self) -> Self::Client;
    fn describe(&mut self) -> Self::Data;
}

