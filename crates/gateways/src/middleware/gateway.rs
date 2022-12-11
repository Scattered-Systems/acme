/*
    Appellation: gateway <middleware>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GatewayMiddleware {
    pub handle: String,
}

impl GatewayMiddleware {
    pub fn new(handle: String) -> Self {
        Self { handle }
    }
}

impl Default for GatewayMiddleware {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
