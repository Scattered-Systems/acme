/*



    Module Requirements:
        - Describe the operations that may be applied to a block
        -
 */
use serde::{Deserialize, Serialize};

use crate::{
    types::{BlockData, BlockHash, BlockId, BlockNonce, LocalTime, TimeStamp},
    utils::timestamp,
};

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
        let timestamp: TimeStamp = LocalTime::now().into();
        Self {
            id,
            data,
            hash: String::from(""),
            nonce,
            previous,
            timestamp
        }
    }
}