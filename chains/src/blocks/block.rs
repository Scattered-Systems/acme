/*
    Appellation: block
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use serde::{Deserialize, Serialize};
use sha2::Digest;
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
        let (nonce, hash) = construct_block(data.clone(), id, previous.clone(), timestamp.clone());

        Self { id, data, hash, nonce, previous, timestamp }
    }
    pub fn consensus() -> Self {
        todo!()
    }
}

pub fn compute_hash_binary_repr(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

pub fn determine_block_validity(block: &Block, previous_block: &Block) -> bool {
    if block.previous != previous_block.hash {
        log::warn!("block with id: {} has wrong previous hash", block.id);
        return false;
    } else if !compute_hash_binary_repr(
        &hex::decode(&block.hash).expect("can decode from hex"),
    )
        .starts_with(crate::DIFFICULTY_PREFIX)
    {
        log::warn!("block with id: {} has invalid difficulty", block.id);
        return false;
    } else if block.id != previous_block.id + 1 {
        log::warn!(
            "block with id: {} is not the next block after the latest: {}",
            block.id, previous_block.id
        );
        return false;
    } else if hex::encode(compute_hash(
        block.id.clone(),
        block.timestamp.clone(),
        block.previous.clone(),
        block.data.clone(),
        block.nonce.clone(),
    )) != block.hash
    {
        log::warn!("block with id: {} has invalid hash", block.id);
        return false;
    }
    true
}

pub fn determine_chain_validity(chain: &[Block]) -> bool {
    for i in 0..chain.len() {
        if i == 0 {
            continue;
        }
        let first = chain.get(i - 1).expect("has to exist");
        let second = chain.get(i).expect("has to exist");
        if !determine_block_validity(second, first) {
            return false;
        }
    }
    true
}

pub fn construct_block(data: BlockData, id: BlockId, previous: BlockHash, timestamp: BlockTime) -> (BlockNonce, BlockHash) {
    log::info!("Creating a new block...");
    let mut nonce = 0;
    loop {
        if nonce % 100000 == 0 {
            log::info!("nonce: {}", nonce);
        }
        let hash = compute_hash(id, timestamp, previous.clone(), data.clone(), nonce);
        let binary_hash = compute_hash_binary_repr(&hash);
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

pub fn compute_hash(id: BlockId, timestamp: BlockTime, previous: BlockHash, data: BlockData, nonce: BlockNonce) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous": previous,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = sha2::Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
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