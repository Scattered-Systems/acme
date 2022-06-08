use bson;
use libp2p::{PeerId, identity, noise};
use std::collections::HashMap;

pub const DIFFICULTY_PREFIX: &str = "00";

#[derive(Clone, Debug)]
pub enum Identity {
    Alien(String),
    Client(PeerId),
    Object(bson::oid::ObjectId),
}

#[derive(Clone)]
pub enum Keys {
    Authenticated(noise::AuthenticKeypair<noise::X25519Spec>),
    Standard(identity::Keypair),
}

pub trait Blockchain {
    type Address;
    type Network;

    fn connect(&mut self) -> String;
    fn get(&self) -> String;
}

pub type AuthenticatedStaticKeys = noise::AuthenticKeypair<noise::X25519Spec>;
pub type Clock = bson::DateTime;
pub type Container<T> = HashMap<String, T>;
pub type NetworkId = PeerId;
pub type ObjectId = bson::oid::ObjectId;
pub type StandardError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type StaticKeys = noise::Keypair<noise::X25519Spec>;
pub type Transaction = Container<String>;