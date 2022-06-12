use crate::primitives::{
    identifiers::PeerId,
    keys::{AuthDhKeys, DhKeys, KeySet},
    networking::BoxedTransport
};
use libp2p::{
    Transport,
    core::upgrade,
    self
};

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PeerId,
    pub key: KeySet
}

impl Peer {
    pub fn new() -> Self {
        let key = KeySet::generate_ed25519();
        let id = PeerId::from(key.public().clone());

        Self {
            id: id.clone(),
            key: key.clone()
        }
    }

    pub fn from(key: KeySet) -> Self {
        Self {
            id: PeerId::from(key.public().clone()),
            key: key.clone()
        }
    }
    pub fn authenticate(&self) -> AuthDhKeys {
        let dh_keys = DhKeys::new()
            .into_authentic(&self.key.clone())
            .expect("Signing Error: Failed to sign the static DH KeyPair");
            return dh_keys.clone()
    }

    pub fn build_transport(&self) -> BoxedTransport {
        let transport = libp2p::tcp::TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(libp2p::noise::NoiseConfig::xx(self.authenticate()).into_authenticated())
            .multiplex(libp2p::mplex::MplexConfig::new())
            .boxed();
        return transport
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Peer(id={})", self.id)
    }
}