pub use tokens::*;
pub use users::*;

pub mod accounts;
pub mod tokens;
pub mod users;

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

pub mod constants {}

pub mod types {}

pub mod utils {}