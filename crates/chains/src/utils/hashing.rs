/*
   Appellation: hashing
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{BlockData, BlockHash, BlockId, BlockNonce, BlockTime};

use sha2::Digest;

pub fn compute_hash_binary_repr(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

pub fn calculate_block_hash(
    id: BlockId,
    data: BlockData,
    nonce: BlockNonce,
    previous: BlockHash,
    timestamp: BlockTime,
) -> Vec<u8> {
    let cache = serde_json::json!(
        {
            "id": id,
            "data": data.clone(),
            "nonce": nonce,
            "previous": previous.clone(),
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
        let id: BlockId = 10;
        let data = "test".to_string();
        let nonce: BlockNonce = 890890;
        let previous = "previous_hash".to_string();
        let timestamp: BlockTime = crate::block_ts_utc();
        let hash = calculate_block_hash(id, data, nonce, previous, timestamp);
        assert_eq!(&hash, &hash)
    }
}
