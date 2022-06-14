/*
    Create a fully-equipped block structure with a number of standard functions outlined below...
 */
use serde::{Deserialize, Serialize};

use crate::types::BoxedError;

use super::Block;

pub trait ChainSpec {
    type Configuration;

    fn setup() -> Self;
    fn connect() -> Result<(), BoxedError>;
}


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