/*
    Create a fully-equipped block structure with a number of standard functions outlined below...
 */
use crate::blocks::Block;
use serde::{Deserialize, Serialize};
use std::hash::Hash;


pub trait Chain {
    type Block;
    type Blocks;

    fn setup(&mut self) -> Self;
    fn connect() -> Result<(), Box<dyn std::error::Error>>;
}


#[derive(Clone, Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

