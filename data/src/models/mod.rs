pub use crate::models::{
    accounts::*,
    tokens::*,
    users::*,
};

mod accounts;
mod tokens;
mod users;

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

pub trait Model {
    type Appellation;
    type Connection;
    type Context;
    type Database;
}