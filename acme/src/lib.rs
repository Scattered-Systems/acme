pub mod actors;
pub mod chains;
pub mod components;
pub mod data;

pub mod primitives {
    use bson;
    use libp2p::{PeerId, identity, noise};
    use std::collections::HashMap;

    pub const DIFFICULTY_PREFIX: &str = "00";

    #[derive(Clone, Debug)]
    pub enum Identity {
        Alien(String),
        Client(PeerId),
        Object(bson::oid::ObjectId),
    }

    #[derive(Clone)]
    pub enum Keys {
        Authenticated(noise::AuthenticKeypair<noise::X25519Spec>),
        Standard(identity::Keypair),
    }

    pub trait Blockchain {
        type Address;
        type Network;

        fn connect(&mut self) -> String;
        fn get(&self) -> String;
    }

    pub type AuthenticatedStaticKeys = noise::AuthenticKeypair<noise::X25519Spec>;
    pub type Clock = bson::DateTime;
    pub type Container<T> = HashMap<String, T>;
    pub type NetworkId = PeerId;
    pub type ObjectId = bson::oid::ObjectId;
    pub type StandardError = Box<dyn std::error::Error + Send + Sync + 'static>;
    pub type StaticKeys = noise::Keypair<noise::X25519Spec>;
    pub type Transaction = Container<String>;
}

pub mod utils {
    use chrono;
    use crate::primitives::{Clock, DIFFICULTY_PREFIX};
    use log::info;
    use serde_json::json;
    use sha2::{Digest, Sha256};

    pub fn hash_to_binary_representation(hash: &[u8]) -> String {
        let mut res: String = String::default();
        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res
    }
    
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

    pub fn timestamp() -> Clock {
        let current_time: Clock = chrono::Local::now().into();
        return current_time
    }
}