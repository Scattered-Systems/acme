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
    pub blockchain: Blockchain,
    pub network: String,
}


#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}