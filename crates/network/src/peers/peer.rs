/*
    Appellation: Peer
    Context: Module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Implements the standard peer structure
*/
use crate::{create_auth_noise_keys, AuthNoiseKey, PeerId, PeerKey};

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
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Peer(id={})", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peers() {
        let peer = Peer::new();
        let from_peer = Peer::from(peer.key.clone());
        assert_eq!(&peer.id, &from_peer.id)
    }
}
