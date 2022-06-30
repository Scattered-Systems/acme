use crate::BoxedTransport;

#[derive(Debug)]
pub struct Provider {
    pub transport: BoxedTransport,
}

impl Provider {
    pub fn new(transport: BoxedTransport) -> Self {
        Self { transport }
    }

    pub fn from(peer: crate::Peer) -> Self {
        Self::new(crate::create_tokio_tcp_transport(peer.authenticate()))
    }
}
