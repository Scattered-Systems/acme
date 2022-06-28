/*
   Appellation: blocks
   Context: module
   Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
   Description:
       ... Summary ...
*/
pub use block::*;

mod block;

pub trait BlockSpec {
    type Data;
    type Index;
    type Hash;
    type Nonce;
    type Timestamp;
    type Transaction;

    fn consensus(
        previous: Self::Hash,
        timestamp: Self::Timestamp,
        transaction: Self::Transaction,
    ) -> Self::Nonce;
    fn constructor(
        id: Self::Index,
        data: Self::Data,
        nonce: Self::Nonce,
        previous: Self::Hash,
    ) -> Self;
    fn determination(&self, block: Self, previous_block: &Self) -> bool;
}

pub fn create_block(
    data: crate::BlockData,
    id: crate::BlockId,
    previous: crate::BlockHash,
    timestamp: crate::BlockTime,
) -> (crate::BlockNonce, crate::BlockHash) {
    log::info!("Creating a new block...");
    let mut nonce = 0;
    loop {
        if nonce % 100000 == 0 {
            log::info!("nonce: {}", nonce);
        }
        let hash =
            crate::calculate_block_hash(id, data.clone(), nonce, previous.clone(), timestamp);
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
