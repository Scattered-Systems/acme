/*
   Appellation: acme
   Context: Library
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
*/
#[doc(inline)]
#[cfg(feature = "chains")]
pub use acme_chains as chains;
#[doc(inline)]
#[cfg(feature = "core")]
pub use acme_core::*;
#[doc(inline)]
#[cfg(feature = "data")]
pub use acme_data as data;
#[doc(inline)]
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use acme_macros::*;
#[doc(inline)]
#[cfg(feature = "network")]
pub use acme_network as net;

pub trait Interface {
    type Actor;
    type Client;
    type Context;
    type Data;

    fn authenticate(&self, actor: Self::Actor) -> bool;
    fn constructor(&self, data: Self::Data) -> Self;
}

pub struct Session {
    pub timestamp: i64,
}

pub struct AppController {
    mode: String,
    name: String,
    session: String,
}
