/*
   Appellation: block
   Context:
   Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
   Description:
       ... Summary ...
*/
use crate::{create_block, BlockData, BlockHash, BlockId, BlockNonce, BlockTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Block {
    pub id: BlockId,
    pub data: BlockData,
    pub hash: BlockHash,
    pub nonce: BlockNonce,
    pub previous: BlockHash,
    pub timestamp: BlockTime,
}

impl Block {
    pub fn new(id: BlockId, data: BlockData, previous: BlockHash) -> Self {
        Block::constructor(id, data, previous)
    }
    pub fn consensus(
        data: BlockData,
        id: BlockId,
        previous: BlockHash,
        timestamp: BlockTime,
    ) -> (BlockNonce, BlockHash) {
        create_block(data, id, previous, timestamp)
    }
    pub fn constructor(id: BlockId, data: BlockData, previous: BlockHash) -> Self {
        let timestamp = crate::Timestamp::utc();
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
        let previous_hash = "".to_string();
        let data = "".to_string();
        let block = Block::new(id, data.clone(), previous_hash.clone());
        assert_eq!(&block, &block)
    }
}
