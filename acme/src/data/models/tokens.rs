/*
    Appellation: tokens <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
/// Captures the different types of authentication tokens typically used with the named organization
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AuthenticationToken {
    OAuth(OAuthToken),
}

/// Defines the standard authentication token for OAuth2.0 enabled systems
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
    pub fn endpoint(url: String) -> String {
        format!("{}/oauth/token", url)
    }
}
