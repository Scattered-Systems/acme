use crate::{BoxedTransport, Peer};

#[derive(Debug)]
pub struct Provider {
    pub peer: Peer,
    pub transport: BoxedTransport,
}

impl Provider {
    pub fn new(peer: &Peer) -> Self {
        Self { peer: peer.clone(), transport: Peer::build_transport(&peer) }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Provider(peers=[{}])", self.peer)
    }
}