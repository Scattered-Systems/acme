/*
    Appellation: Peers
    Context: Module
    Creator: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
    Description:
 */
pub use peer::*;

mod peer;

pub type PeerError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait PeerSpec<E> {
    type Actor;
    type Conduit;
    type Configuration;
    type Data;

    fn configure(&self, configuration: Self::Configuration) -> Result<Self, E> where Self: Sized;
}

