/*
    Description: Create a standard structure and implement the desired specifications
 */

use crate::{
    blockchain::{Block, ChainSpec}, Deserialize,
    Serialize,
    types::BoxedError,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chain {
    blocks: Vec<Block>,
}

impl ChainSpec for Chain {
    type Configuration = ();

    fn setup() -> Self {
        todo!()
    }

    fn connect() -> Result<(), BoxedError> {
        todo!()
    }
}