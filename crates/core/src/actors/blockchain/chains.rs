use super::Block;
use crate::BoxedError;

use serde::{Deserialize, Serialize};

pub trait StandardChainSpec {
    type Block;
    type Configuration;

    fn activate(configuration: Self::Configuration) -> Self;
    fn client() -> Result<(), BoxedError>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chain {
    blocks: Vec<Block>,
}

impl StandardChainSpec for Chain {
    type Block = ();
    type Configuration = ();

    fn activate(configuration: Self::Configuration) -> Self {
        todo!()
    }

    fn client() -> Result<(), BoxedError> {
        todo!()
    }
}
