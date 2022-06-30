/*
    Appellation: Providers
    Context: Module
    Creator: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
    Description:

 */
use libp2p::{Transport, core::upgrade};

pub use provider::*;

mod provider;

#[derive(Clone, Debug)]
pub enum Providers {
    Controller {
        peers: Vec<crate::Peer>
    }
}

pub trait ProviderSpec {
    type Actor;
    type Config;
    type Context;
    type Data;

    fn new(config: Self::Config) -> Self;
    fn from(config: Self::Config) -> Self;
}

pub fn create_tokio_tcp_transport(dh_keys: crate::AuthNoiseKey) -> crate::BoxedTransport {
    libp2p::tcp::TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(libp2p::noise::NoiseConfig::xx(dh_keys).into_authenticated())
        .multiplex(libp2p::mplex::MplexConfig::new())
        .boxed()
}