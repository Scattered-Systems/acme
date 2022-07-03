/*
   Appellation: accounts
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use account::*;
pub use wallet::*;

mod account;
mod wallet;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ErcTokens {
    ER20 {
        name: String,
        symbol: String,
        total_supply: String,
    },
    ERC721,
    ERC1155,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AccountStates {
    Active,
    Inactive,
}

pub trait AccountSpec: Sized {
    type Id;
    type Address;
    type Username;
    type Password;
    type Timestamp;
}