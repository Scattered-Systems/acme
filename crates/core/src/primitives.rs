/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {
    ///
    pub const ACCEPT_APP_JSON: &str = "application/json";
    ///
    pub const ME_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59";
    ///
    pub const LOCALHOST: [u8; 4] = [127, 0, 0, 1];
}

pub(crate) mod types {
    ///
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
    ///
    pub type BoxError = Box<dyn std::error::Error>;
}
