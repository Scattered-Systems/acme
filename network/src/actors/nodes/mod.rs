/*
   Appellation: Nodes
   Context: Module
   Creator: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
   Description:

*/
pub use node::*;

mod node;

type NodeError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub enum NodePartitions {

}

pub trait NodeSpec<Address, Configuration, Context, Data> {
    fn authenticate(&self, context: Context) -> bool where Self: Sized;
    fn configure(&self, configuration: Configuration) -> Result<Self, config::ConfigError> where Self: Sized;
}