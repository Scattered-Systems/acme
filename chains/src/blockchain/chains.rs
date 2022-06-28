/*
    Appellation: chains
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
use crate::Block;
use serde::{Deserialize, Serialize};

pub type Blockchain = Vec<Block>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chain {
    pub blockchain: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            blockchain: Vec::new()
        }
    }

    pub fn constructor(&mut self) -> Vec<Block> {
        let id = 0;
        let data = "".to_string();
        let previous_hash = "genesis".to_string();
        let genesis_block = Block::new(data.clone(), id, previous_hash.clone());
        self.blockchain.push(genesis_block);
        self.blockchain.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }

    #[test]
    fn test_chain_genesis() {
        let mut chain = Chain::new();
        let blockchain = Chain::constructor(&mut chain);
        println!("{:#?}", &chain);
        assert_eq!(&blockchain, &blockchain)
    }
}