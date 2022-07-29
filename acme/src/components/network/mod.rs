/*
    Appellation: network
    Context: module
    Description:
        ... Summary ...
*/
pub use proxy::*;
pub use router::*;
pub use server::*;

mod proxy;
mod router;
mod server;

/// Implements a trait enabling networking components to be addressable
pub trait Addressable {
    fn host(&self) -> [u8; 4]
        where
            Self: Sized;
    fn port(&self) -> u16
        where
            Self: Sized;
    fn address_from(&mut self) -> std::net::SocketAddr
        where
            Self: Sized,
    {
        std::net::SocketAddr::from((self.host(), self.port()))
    }
}
