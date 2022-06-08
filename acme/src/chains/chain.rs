use crate::chains::block::Block;
use crate::primitives::Transaction;

#[derive(Clone, Debug)]
pub struct Chain {
    blocks: Vec<Block>
}

impl Chain {
    pub fn new() -> Self {
        let hash = "".to_string();
        let nonce = "".to_string();
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