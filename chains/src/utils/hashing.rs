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
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
