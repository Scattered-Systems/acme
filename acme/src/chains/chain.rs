/*
    Describe the general structure and functionality of a blockchain
 */

use crate::{chains::block::Block, primitives::containers::Transaction};

#[derive(Clone, Debug)]
pub struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        let hash = "".to_string();
        let nonce: usize = 98078789;
        let previous = "".to_string();

        let mut tmp = Transaction::new();
        tmp.insert(String::from("label"), String::from("Foundation"));
        let transactions = vec![tmp];
        let beta = Block::new(nonce, previous, transactions);
        Self {
            blocks: vec![beta]
        }
    }
}