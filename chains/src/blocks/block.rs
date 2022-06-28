/*
    Appellation: block
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

use serde::{Deserialize, Serialize};

use crate::{BlockData, BlockHash, BlockId, BlockNonce, Timestamp};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Block {
    pub id: BlockId,
    pub data: BlockData,
    pub hash: BlockHash,
    pub nonce: BlockNonce,
    pub previous: BlockHash,
    pub timestamp: Timestamp,
}

impl Block {
    pub fn new(data: BlockData, nonce: BlockNonce, previous: BlockHash) -> Self {
        let id = BlockId::new();
        let hash: BlockHash = "".to_string();
        let timestamp = crate::utils::timestamp();

        Self { id, data, hash, nonce, previous, timestamp }
    }
    pub fn consensus() -> Self {
        todo!()
    }
}
