pub use crate::models::{
    accounts::*,
    tokens::*,
    users::*,
    wallet::*
};

mod accounts;
mod tokens;
mod users;
mod wallet;

pub enum DatabaseClassifications {
    Centralized,
    Decentralized,
    Distributed,
    SelfHosted,
}

pub enum DatabaseStyles {
    Object,
    NoSQL,
    SQL,
}

pub trait ModelSpec {
    type Actor;
    type Client;
    type Context;
    type Data;

    fn configure(&self, context: Self::Context) -> Self::Actor;
    fn constructor(&self, data: Self::Data) -> Self;
}