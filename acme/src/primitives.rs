use std::collections::HashMap;

use bson;
use libp2p::{identity, noise, PeerId};

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

pub mod containers {
    use std::collections::HashMap;

    pub type Container<T> = HashMap<String, T>;
    pub type Transaction = Container<String>;
}

pub mod errors {
    use std::error::Error;

    pub type AsyncError = Box<dyn Error + Send + Sync + 'static>;
}

pub mod identifiers {
    use bson;
    use libp2p;

    pub type NetworkAddress = libp2p::Multiaddr;
    pub type NetworkId = libp2p::PeerId;
    pub type ObjectId = bson::oid::ObjectId;
}

pub mod keys {
    use libp2p::{identity, noise};

    pub type AuthenticKey = noise::AuthenticKeypair<noise::X25519Spec>;
    pub type MasterKey = identity::Keypair;
    pub type NoiseKey = noise::Keypair<noise::X25519Spec>;
}