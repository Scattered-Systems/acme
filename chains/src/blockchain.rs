/*
    Appellation: blockchain
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
    pub chain: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            chain: Vec::new()
        }
    }

    pub fn constructor(&self) -> Self {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}