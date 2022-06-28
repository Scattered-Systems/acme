use crate::{AuthNoiseKey, BoxedTransport, NoiseKey, PeerId, PeerKey};
use libp2p::{Transport, core::upgrade};

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PeerId,
    pub key: PeerKey,
}

impl Peer {
    pub fn new() -> Self {
        let key = PeerKey::generate_ed25519();
        let id = PeerId::from(key.public().clone());

        Self {
            id: id.clone(),
            key: key.clone(),
        }
    }

    pub fn from(key: PeerKey) -> Self {
        Self {
            id: PeerId::from(&key.public()),
            key,
        }
    }

    pub fn authenticate(&self) -> AuthNoiseKey {
        create_auth_noise_keys(&self.key)
    }

    pub fn build_transport(&self) -> BoxedTransport {
        construct_transport(self.authenticate())
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Peer(id={})", self.id)
    }
}

pub fn create_auth_noise_keys(key: &PeerKey) -> AuthNoiseKey {
    NoiseKey::new()
        .into_authentic(&key)
        .expect("Authentication Error: Failed to sign the provided keypair")
}

pub fn construct_transport(dh_keys: AuthNoiseKey) -> BoxedTransport {
    libp2p::tcp::TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(libp2p::noise::NoiseConfig::xx(dh_keys).into_authenticated())
        .multiplex(libp2p::mplex::MplexConfig::new())
        .boxed()
}