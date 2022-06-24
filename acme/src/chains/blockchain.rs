use serde::{Deserialize, Serialize};
use crate::errors::BoxedError;

use super::Block;

pub enum ChainStates {
    Appending,
    Computing,
    Connecting,
    Determining,
}


pub trait ChainSpec {
    type Appellation; // Define the Chain's name
    type Configuration; // Type of configuration for the Chain
    type Context; // Define the
    type Data; // Define the standard data structure to be use throughout the Chain

    fn activate(appellation: Self::Appellation, configuration: Self::Configuration) -> Self;
    fn chain(&self) -> Result<(), BoxedError>;
    fn constructor(&self) -> Self;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blockchain {
    pub appellation: String,
    pub configuration: crate::Container,
    pub context: crate::Container,
    pub data: Vec<Block>,
}

impl ChainSpec for Blockchain {
    type Appellation = String;
    type Configuration = crate::Container;
    type Context = crate::Container;
    type Data = Vec<Block>;

    fn activate(appellation: Self::Appellation, configuration: Self::Configuration) -> Self {
        todo!()
    }

    fn chain(&self) -> Result<(), BoxedError> {
        todo!()
    }

    fn constructor(&self) -> Self {
        todo!()
    }
}

