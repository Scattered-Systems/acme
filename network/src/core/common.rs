/*
   Appellation: common
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use types::*;

mod types {
    pub use libp2p::identity::Keypair as PeerKey;
    pub use libp2p::noise::X25519Spec as CryptoSpec;
    pub use libp2p::Multiaddr as NetworkAddress;
    pub use libp2p::PeerId;
    use libp2p::{
        self,
        core::{muxing::StreamMuxerBox, transport::Boxed},
    };

    // Authenticated DH Keys
    pub type AuthNoiseKey = libp2p::noise::AuthenticKeypair<CryptoSpec>;
    // Boxed Transport
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
    pub type KademliaMS = libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>;
    // Wrapper for Noise Keypair
    pub type NoiseKey = libp2p::noise::Keypair<CryptoSpec>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
