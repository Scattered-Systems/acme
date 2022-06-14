use crate::actors::Peer;
use crate::types::BoxedTransport;

pub use super::specs::Provider as ProviderSpec;

#[derive(Debug)]
pub struct Provider {
    pub peer: Peer,
    pub transport: BoxedTransport,
}

impl Provider {
    pub fn new(peer: &Peer) -> Self {
        let transport = Peer::build_transport(&peer);
        Self { peer: peer.clone(), transport }
    }
}

impl ProviderSpec for Provider {
    fn setup() -> Self {
        todo!()
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Provider(peers=[{}])", self.peer)
    }
}