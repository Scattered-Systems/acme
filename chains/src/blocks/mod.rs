/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
pub use crate::blocks::block::*;

mod block;

pub const DIFFICULTY_PREFIX: &str = "00";

pub trait BlockSpec {
    type Data;
    type Index;
    type Hash;
    type Nonce;
    type Timestamp;
    type Transaction;

    fn actor(&self) -> Self::Hash;
    // Calculate the block hash
    fn consensus(&self) -> Self;
    // Implement the block's consensus mechanism
    fn constructor(&self, data: Self::Data, nonce: Self::Nonce, previous: Self::Hash) -> Self;
    // Fetch a block
    fn validator(&self) -> Self::Nonce;
}

pub type BlockData = String;
pub type BlockHash = String;
pub type BlockId = bson::oid::ObjectId;
pub type BlockNonce = u64;
pub type Timestamp = bson::DateTime;