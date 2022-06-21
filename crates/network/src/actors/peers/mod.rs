/*
    Appellation: Peers
    Context: Module
    Creator: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
    Description:
 */
mod peer;
pub use peer::*;


pub type PeerError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait PeerSpec {
    type Appellation;
    type Configuration;
    type Container;
    type Data;

    fn actor(&self) -> Result<Self::Container, PeerError>;
    fn configure(&self, configuration: Self::Configuration) -> Self;
    fn constructor(configuration: Self::Configuration) -> Self;
    fn datastore(data: Self::Data) -> Result<Self::Container, PeerError>;
}

