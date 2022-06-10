/*
    Create a fully-equipped block structure with a number of standard functions outlined below...
 */
use crate::{
    primitives::{containers::Transaction, identifiers::ObjectId, date::Stamp},
    utils::date::timestamp,
};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    pub id: ObjectId,
    pub hash: String,
    pub nonce: usize,
    pub previous: String,
    pub timestamp: Stamp,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(nonce: usize, previous: String, transactions: Vec<Transaction>) -> Self {
        Self {
            id: ObjectId::new(),
            hash: String::from(""),
            nonce,
            previous,
            timestamp: timestamp(),
            transactions,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

pub mod utils {
    use crate::primitives::constants::DIFFICULTY_PREFIX;
    use log::info;
    use serde_json::json;
    use sha2::{Digest, Sha256};

    // Cacluate the hash of a Block using standard Block parameters
    pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
        let data = json!({
            "id": id,
            "previous_hash": previous_hash,
            "data": data,
            "timestamp": timestamp,
            "nonce": nonce
        });
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }

    // Represent a Block hash in the proper format for the binary interface 
    pub fn hash_to_binary_representation(hash: &[u8]) -> String {
        let mut res: String = String::default();
        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res
    }

    // Defines the standard method in which blocks are to be mined
    pub fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        info!("mining block...");
        let mut nonce = 0;

        loop {
            if nonce % 100000 == 0 {
                info!("nonce: {}", nonce);
            }
            let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
            let binary_hash = hash_to_binary_representation(&hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                info!(
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
}