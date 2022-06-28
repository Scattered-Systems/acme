/*
    Appellation: acme-network
    Creator: FL03 <jo3mccain@icloud.com>
    Context:
    Description:
 */
pub use crate::{
    actors::*,
    behaviours::*,
    common::*,
    contracts::*,
};

mod actors;
mod behaviours;
mod contracts;

pub mod common {
    pub use types::*;

    mod types {
        use libp2p::{self, core::{muxing::StreamMuxerBox, transport::Boxed}};
        pub use libp2p::identity::Keypair as PeerKey;
        pub use libp2p::Multiaddr as NetworkAddress;
        pub use libp2p::noise::X25519Spec as CryptoSpec;
        pub use libp2p::PeerId;

        // Authenticated DH Keys
        pub type AuthNoiseKey = libp2p::noise::AuthenticKeypair<CryptoSpec>;
        // Boxed Transport
        pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
        pub type KademliaMS = libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>;
        // Wrapper for Noise Keypair
        pub type NoiseKey = libp2p::noise::Keypair<CryptoSpec>;
    }
}
