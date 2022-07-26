/*
    Appellation: tokens <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub enum AuthToken {
    OAuth(OAuthToken),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OAuthToken {
    pub access_type: Vec<String>,
    pub token: String,
}

impl OAuthToken {
    pub fn new() -> Self {
        Self {
            access_type: Vec::<String>::new(),
            token: String::new(),
        }
    }
}
