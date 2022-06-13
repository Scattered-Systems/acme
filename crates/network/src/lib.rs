pub mod behaviours;
pub mod nodes;
pub mod peers;
pub mod providers;
pub mod utils;

pub mod types {
    use libp2p::{core::{muxing::StreamMuxerBox, transport::Boxed}, self};

    // Networking
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
    pub type NetworkAddress = libp2p::Multiaddr;
    pub type PeerId = libp2p::PeerId;
    pub type PeerKey = libp2p::identity::Keypair;

    // Noise Keys
    pub type CryptoSpec = libp2p::noise::X25519Spec;

    pub type AuthNoiseKey = libp2p::noise::AuthenticKeypair<CryptoSpec>; // Authenticated DH Keys
    pub type NoiseKey = libp2p::noise::Keypair<CryptoSpec>; // Standard DH Keys
}