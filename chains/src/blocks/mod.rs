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

    // Describe the consensus mechanism for determining the validity of a transaction
    fn consensus(
        previous: Self::Hash,
        timestamp: Self::Timestamp,
        transaction: Self::Transaction,
    ) -> Self::Nonce;
    // Describes the creation of a block
    fn constructor(&self, data: Self::Data, nonce: Self::Nonce, previous: Self::Hash) -> Self;
}