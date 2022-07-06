/*
   Appellation: blocks
   Context: module
   Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
   Description:
       ... Summary ...
*/
pub use block::*;
pub use utils::*;

mod block;

pub type BoxedBlock = Box<dyn AbstractBlock>;

pub trait AbstractBlock<Data = String, Index = u8, Hash = String, Nonce = u8, Ts = i64> {
    fn constructor(
        &self,
        id: Index,
        hash: Hash,
        nonce: Nonce,
        previous: Hash,
        timestamp: Ts,
    ) -> Self
        where
            Self: Sized;
}

pub trait BlockSpec<Dt = String, Id = u8, Hs = Vec<u8>, Nc = u8, Ts = i64> {
    fn constructor(
        &self,
        id: Id,
        hash: Hs,
        nonce: Nc,
        previous: Hs,
        timestamp: Ts,
        transactions: Vec<Dt>,
    ) -> Self
    where
        Self: Sized;
    fn consensus(
        &self,
        id: Id,
        hash: Hs,
        previous: Hs,
        timestamp: Ts,
        transactions: Vec<Dt>,
    ) -> (Nc, Hs)
    where
        Self: Sized;
}

mod utils {
    use crate::{BData, BHash, BId, BNonce, BTStamp, DIFFICULTY_PREFIX};
    use log;
    use sha2::Digest;

    pub fn create_block(
        data: BData,
        id: BId,
        previous: BHash,
        timestamp: BTStamp,
    ) -> (BNonce, BHash) {
        log::info!("Creating a new block...");
        let mut nonce = 0;
        loop {
            if nonce % 100000 == 0 {
                log::info!("nonce: {}", nonce);
            }
            let hash = calculate_block_hash(id, data.clone(), nonce, previous.clone(), timestamp);
            let binary_hash = convert_hash_into_binary(&hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX.as_ref()) {
                log::info!(
                    "mined! nonce: {}, hash: {}, binary hash: {:#?}",
                    nonce,
                    hex::encode(&hash),
                    binary_hash
                );
                return (nonce, hex::encode(hash).into());
            }
            nonce += 1;
        }
    }

    pub fn convert_hash_into_binary(hash: &[u8]) -> Vec<u8> {
        let mut res: String = String::default();
        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res.into_bytes()
    }

    pub fn calculate_block_hash(
        id: BId,
        data: BData,
        nonce: BNonce,
        previous: BHash,
        timestamp: BTStamp,
    ) -> Vec<u8> {
        let cache = serde_json::json!(
            {
                "id": id,
                "data": data.clone(),
                "nonce": nonce,
                "previous": previous,
                "timestamp": timestamp
            }
        );
        let mut hasher = sha2::Sha256::new();
        hasher.update(cache.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_block_hash() {
            let id: BId = 10;
            let data = "test".to_string();
            let nonce: BNonce = 890890;
            let previous = "previous_hash".to_string();
            let timestamp: BTStamp = crate::block_ts_utc();
            let hash = calculate_block_hash(id, data.clone(), nonce, previous.clone(), timestamp);
            assert_eq!(&hash, &hash)
        }
    }
}
