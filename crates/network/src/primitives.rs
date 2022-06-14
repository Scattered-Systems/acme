pub mod constants {
    pub use crate::actors::constants::*;
    pub use crate::behaviours::constants::*;
    pub use crate::crypto::constants::*;
}

pub mod types {
    use libp2p::{self, core::{muxing::StreamMuxerBox, transport::Boxed}};

    pub use crate::actors::constants::*;
    pub use crate::behaviours::types::*;
    pub use crate::crypto::types::*;

    pub type AuthNoiseKey = libp2p::noise::AuthenticKeypair<CryptoSpec>;
    // Authenticated DH Keys
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
    // Transport Instances
    pub type CryptoSpec = libp2p::noise::X25519Spec;
    // Standard Network Encryption
    pub type NetworkAddress = libp2p::Multiaddr;
    // Wrapper for libp2p::Multiaddr
    pub type NoiseKey = libp2p::noise::Keypair<CryptoSpec>;
    pub type PeerId = libp2p::PeerId;
    // Wrapper for libp2p::PeerId
    pub type PeerKey = libp2p::identity::Keypair;
}

