/*
   Appellation: block
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Implement the standard block structure
*/
use crate::{create_block, BData, BHash, BId, BNonce, BTStamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Block {
    pub id: BId,
    pub data: BData,
    pub hash: BHash,
    pub nonce: BNonce,
    pub previous: BHash,
    pub timestamp: BTStamp,
}

impl Block {
    pub fn new(id: BId, data: BData, previous: BHash) -> Self {
        Block::constructor(id, data, previous)
    }
    pub fn consensus(data: BData, id: BId, previous: BHash, timestamp: BTStamp) -> (BNonce, BHash) {
        create_block(data, id, previous, timestamp)
    }
    pub fn constructor(id: BId, data: BData, previous: BHash) -> Self {
        let timestamp = crate::BlockStamp::utc();
        let (nonce, hash) = Block::consensus(data.clone(), id, previous.clone(), timestamp);
        Self {
            id,
            data,
            hash,
            nonce,
            previous,
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curate_block() {
        let id = 0;
        let previous: BHash = "".into();
        let data = "".to_string();
        let block = Block::new(id, data.clone(), previous);
        assert_eq!(&block, &block)
    }
}
