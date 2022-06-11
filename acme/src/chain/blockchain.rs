/*
    Create a fully-equipped block structure with a number of standard functions outlined below...
 */
use crate::primitives::date::{Local, Stamp};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct Block {
    pub id: types::BlockId,
    pub data: types::Data,
    pub hash: types::BlockHash,
    pub nonce: types::Nonce,
    pub previous: types::BlockHash,
    pub timestamp: Stamp,
}

impl Block {
    pub fn new(data: types::Data, nonce: types::Nonce, previous: String) -> Self {
        let id = types::BlockId::new();
        let timestamp: Stamp = Local::now().into();
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

#[derive(Clone, Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

pub mod types {
    use crate::primitives::{containers::Container, identifiers::ObjectId};

    pub type BlockId = ObjectId;
    pub type BlockHash = String;
    pub type Data = String;
    pub type Nonce = u64;
    pub type Transaction = Container<String>;
}

pub mod utils {
    use crate::{
        chain::blockchain::types,
        primitives::{constants::DIFFICULTY_PREFIX, date::Stamp}
    };
    use log::info;
    use serde_json::json;
    use sha2::{Digest, Sha256};

    // Cacluate the hash of a Block using standard Block parameters
    pub fn calculate_hash(id: types::BlockId, data: types::Data, nonce: types::Nonce, previous: types::BlockHash, timestamp: Stamp) -> Vec<u8> {
        let data = json!({
            "id": id,
            "data": data,
            "nonce": nonce,
            "previous": previous,
            "timestamp": timestamp
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
    pub fn mine_block(id: types::BlockId, data: types::Data,  previous: types::BlockHash, timestamp: Stamp) -> (u64, String) {
        info!("mining block...");
        let mut nonce = 0;

        loop {
            if nonce % 100000 == 0 {
                info!("nonce: {}", nonce);
            }
            let hash = calculate_hash(id, data.clone(), nonce, previous.clone(), timestamp);
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