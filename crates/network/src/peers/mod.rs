/*
    Appellation: Peers
    Context: module
    Created: FL03 <jo3mccain@icloud.com> [06-30-2022]
    Description:
        Outlines a number of standards for creating and managing peers
*/
pub use peer::*;

mod peer;

pub fn create_auth_noise_keys(key: &crate::PeerKey) -> crate::AuthNoiseKey {
    crate::NoiseKey::new()
        .into_authentic(&key)
        .expect("Authentication Error: Failed to sign the provided keypair")
}
