/*
    Appellation: provider
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated to implementing the standard Provider structure suitable for a
        majority of the requests
 */
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

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}