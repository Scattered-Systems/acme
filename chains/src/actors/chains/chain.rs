/*
   Appellation: chains
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::Block;
use serde::{Deserialize, Serialize};

type Blockchain = Vec<Block>;

pub enum ChainStates {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chain {
    pub blockchain: Blockchain,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            blockchain: Vec::new(),
        }
    }

    pub fn constructor(&mut self) -> Blockchain {
        let id = 0;
        let data = "".to_string();
        let previous = "genesis".to_string();
        let genesis_block = Block::new(id, data.clone(), previous.clone());
        self.blockchain.push(genesis_block);
        self.blockchain.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_genesis() {
        let mut chain = Chain::new();
        let blockchain = Chain::constructor(&mut chain);
        println!("{:#?}", &chain);
        assert_eq!(&blockchain, &blockchain)
    }
}
