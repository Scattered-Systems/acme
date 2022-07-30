/*
   Appellation: primitives
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use constants::*;
pub use types::*;

mod constants {
    ///
    pub const COINBASE_API_ENDPOINT: &str = "https://coinbase.com/api/v2";
    ///
    pub const COINBASE_PRO_ENDPOINT: &str = "https://pro.coinbase.com/api";
    ///
    pub const STANDARD_OAUTH_TOKEN_PATH: &str = "oauth/token";
}

mod types {
    /// Describes a boxed dynamic error with Send, Sync and 'static tags enabled
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
}
