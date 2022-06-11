pub mod constants {
    pub const DIFFICULTY_PREFIX: &str = "00";
}

pub mod containers {
    use std::collections::HashMap;

    pub type Container<T> = Dict<Vec<T>>;
    pub type Dict<T> = HashMap<String, T>;
}

pub mod date {
    use bson;
    use chrono;

    pub type DateTime<T> = chrono::DateTime<T>;
    pub type Local = chrono::Local;
    pub type Stamp = bson::DateTime;
}

pub mod errors {
    pub type BoxedError = Box<dyn std::error::Error>;
    pub type DynamicError = Box<dyn std::error::Error + Send + Sync + 'static>;
}

pub mod identifiers {
    use bson;
    use libp2p;

    
    pub type ContentId = String;
    pub type NetworkAddress = libp2p::Multiaddr;
    pub type ObjectId = bson::oid::ObjectId;
    pub type PeerId = libp2p::PeerId;
}

pub mod keys {
    use libp2p;

    pub type CryptoSpec = libp2p::noise::X25519Spec;

    pub type AuthDhKeys = libp2p::noise::AuthenticKeypair<CryptoSpec>; // Authenticated DH Keys
    pub type DhKeys = libp2p::noise::Keypair<CryptoSpec>; // Standard DH Keys
    pub type KeySet = libp2p::identity::Keypair;
}



pub mod networking {
    use crate::primitives::identifiers::PeerId;
    use libp2p::core::{muxing::StreamMuxerBox, transport::Boxed};

    
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
}

pub mod states {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum Validator {
        Invalid,
        Valid,
        Validating,
    }
}