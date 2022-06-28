/*
    Appellation: block
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use serde::{Deserialize, Serialize};
use crate::{BlockData, BlockHash, BlockId, BlockNonce, BlockTime};

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
    pub fn new(data: BlockData, id: BlockId, previous: BlockHash) -> Self {
        let timestamp = crate::utils::timestamp();
        let (nonce, hash) = create_block(data.clone(), id, previous.clone(), timestamp.clone());

        Self { id, data, hash, nonce, previous, timestamp }
    }
    pub fn consensus(&self, data: BlockData, id: BlockId, previous: BlockHash, timestamp: BlockTime) -> (BlockNonce, BlockHash) {
        create_block(data, id, previous, timestamp)
    }
}


pub fn create_block(data: BlockData, id: BlockId, previous: BlockHash, timestamp: BlockTime) -> (BlockNonce, BlockHash) {
    log::info!("Creating a new block...");
    let mut nonce = 0;
    loop {
        if nonce % 100000 == 0 {
            log::info!("nonce: {}", nonce);
        }
        let hash = crate::calculate_block_hash(
            data.clone(),
            id,
            nonce,
            previous.clone(),
            timestamp.clone(),
        );
        let binary_hash = crate::compute_hash_binary_repr(&hash);
        if binary_hash.starts_with(crate::DIFFICULTY_PREFIX) {
            log::info!(
                "mined! nonce: {}, hash: {}, binary hash: {}",
                nonce,
                hex::encode(&hash),
                binary_hash
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
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
        let block = Block::new(data.clone(), id, previous_hash.clone());
        println!("{:#?}", &block);
        assert_eq!(&block, &block)
    }
}