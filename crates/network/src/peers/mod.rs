/*
    Appellation: Peers
    Context: module
    Created: FL03 <jo3mccain@icloud.com> [06-30-2022]
    Description:
        Outlines a number of standards for creating and managing peers
*/
pub use peer::*;

mod peer;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PeerState {
    state: String,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PeerStates {
    Inoperable(PeerState),
    Operating(PeerState),
    Operational(PeerState),
}

#[derive(Clone, Debug)]
pub enum Peers {
    Standard(Peer),
}

pub trait PeerSpec: Sized {
    type Address;
    type Id;
    type Key;

    fn authenticate(&self) -> Self;
    fn constructor(id: Self::Id, key: Self::Key) -> Self;
    fn new() -> Self;
    fn from(key: Self::Key) -> Self;
}

pub fn create_auth_noise_keys(key: &crate::PeerKey) -> crate::AuthNoiseKey {
    crate::NoiseKey::new()
        .into_authentic(&key)
        .expect("Authentication Error: Failed to sign the provided keypair")
}
