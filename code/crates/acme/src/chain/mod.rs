pub mod blockchain;
pub mod networking;

pub mod constants {
    pub const EPOCH: usize = 30;
}

pub mod types {
    use crate::primitives::types::*;

    pub type BlockData = String;
    pub type BlockId = ObjectId;
    pub type BlockHash = String;
    pub type BlockNonce = usize;
}