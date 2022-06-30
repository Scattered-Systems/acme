/*
    Appellation: wallet
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use bson::oid::ObjectId;

type Secret = Vec<String>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AccessGrant {
    pub key: String,
    pub data: Vec<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Account {
    pub address: String,
    pub alias: String,
    pub key: String,
    pub secret: Secret,
    pub token: String,
}

impl Account {
    pub fn new(address: String, alias: String, key: String, secret: Secret, token: String) -> Self {
        Self { address, alias, key, secret, token }
    }
    pub fn from(address: &str, alias: &str, key: &str, token: &str) -> Self {
        Self::new(
            String::from(address),
            String::from(alias),
            String::from(key),
            Secret::new(),
            String::from(token),
        )
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Wallet {
    pub accounts: Vec<Account>,
}

impl Wallet {
    pub fn setup(address: String, alias: String, key: String, secret: Secret, token: String) -> Self {
        todo!()
    }
    pub fn new() -> Self {
        todo!()
    }
    pub fn from(address: String) -> Self {
        todo!()
    }
}