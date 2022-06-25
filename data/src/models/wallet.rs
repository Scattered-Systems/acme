/*
    Appellation: wallet
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub type AccessGrant = [String; 12];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Profile {
    pub appellation: String,
    pub data: Vec<crate::Container>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Account {
    pub access: AccessGrant,
    pub data: [Profile; 3],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Wallet {
    pub accounts: Vec<Account>,
}