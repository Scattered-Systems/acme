pub mod actors;
pub mod chains;
pub mod controllers;
pub mod utils;

pub mod types {
    use crate::chains::types::*;
    use bson;
    use chrono;
    use libp2p::{self, core::{muxing::StreamMuxerBox, transport::Boxed}};

    use std::collections::HashMap;


    // Containers
    pub type Container<T> = Dict<Vec<T>>;
    pub type Dict<T> = HashMap<String, T>;

    // Datetime
    pub type DateTime<T> = chrono::DateTime<T>;
    pub type LocalTime = chrono::Local;
    pub type TimeStamp = bson::DateTime;

    // Errors
    pub type BoxedError = Box<dyn std::error::Error>;
    pub type DynamicError = Box<dyn std::error::Error + Send + Sync + 'static>;

    // Ids
    pub type ContentId = String;
    pub type ObjectId = bson::oid::ObjectId;

    // Networking
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
    pub type NetworkAddress = libp2p::Multiaddr;


    // Noise Keys
    pub type CryptoSpec = libp2p::noise::X25519Spec;

    pub type AuthNoiseKey = libp2p::noise::AuthenticKeypair<CryptoSpec>; // Authenticated DH Keys
    pub type NoiseKey = libp2p::noise::Keypair<CryptoSpec>; // Standard DH Keys

    // Peers
    pub type PeerId = libp2p::PeerId;
    pub type PeerKey = libp2p::identity::Keypair;
}