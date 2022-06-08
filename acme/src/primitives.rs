use bson;
use libp2p::{PeerId, identity, noise};
use std::collections::HashMap;

pub const DIFFICULTY_PREFIX: &str = "00";

pub type AuthenticatedStaticKeys = noise::AuthenticKeypair<noise::X25519Spec>;
pub type Clock = bson::DateTime;
pub type Container<T> = HashMap<String, T>;
pub type MasterKeys = identity::Keypair;
pub type NetworkId = PeerId;
pub type ObjectId = bson::oid::ObjectId;
pub type StandardError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type StaticKeys = noise::Keypair<noise::X25519Spec>;
pub type Transaction = Container<String>;