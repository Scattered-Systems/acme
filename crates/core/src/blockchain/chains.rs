/*
    Create a fully-equipped block structure with a number of standard functions outlined below...
 */
use super::blocks::Block;
use serde::{Deserialize, Serialize};

pub trait Chain<T> {
    type Blocks;

    fn setup(&mut self, settings: T) -> Self;
    fn connect() -> Result<(), crate::types::BoxedError>;
    fn get(&self) -> Self;
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blockchain {
    blocks: Vec<Block>,
}