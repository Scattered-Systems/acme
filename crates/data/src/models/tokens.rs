use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub enum Tokens {
    Auth(String)
}


#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct Token {
    kind: Tokens,
    token: String,
}