/*
    Appellation: blocks
    Creator:
    Concepts:
    Description:
 */
use serde::{Deserialize, Serialize};

use crate::{BlockData, BlockHash, BlockId, BlockNonce, TimeStamp};
use crate::timestamp;

pub trait BlockSpec {
    type Data;
    type Index;
    type Hash;
    type Nonce;
    type Timestamp;
    type Transaction;

    fn actor(&self) -> Self::Hash; // Caclulate the block hash
    fn consensus(&self) -> Self; // Implement the block's consensus mechanism
    fn constructor(&self, data: Self::Data, nonce: Self::Nonce, previous: Self::Hash) -> Self;
    fn descriptor(&self, id: Self::Index) -> Self; // Fetch a block
}

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct Block {
    pub id: BlockId,
    pub data: BlockData,
    pub hash: BlockHash,
    pub nonce: BlockNonce,
    pub previous: BlockHash,
    pub timestamp: TimeStamp,
}

impl Block {
    pub fn new(data: BlockData, nonce: BlockNonce, previous: BlockHash) -> Self {
        let id = BlockId::new();
        let hash: BlockHash = "".to_string();
        let timestamp = timestamp();

        Self { id, data, hash, nonce, previous, timestamp }
    }

    pub fn consensus() -> Self {
        todo!()
    }
}
