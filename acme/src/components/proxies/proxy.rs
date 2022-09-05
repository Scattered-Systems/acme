/*
    Appellation: proxy <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use std::{net::SocketAddr, str::FromStr};



#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Proxy {
    pub address: SocketAddr
}

impl Proxy {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }
    pub fn addr_from_str(addr: &str) -> SocketAddr {
        match SocketAddr::from_str(addr) {
            Ok(v) => v,
            Err(e) => panic!("Conversion Error: {}", e)
        }
    }
    pub fn from_str(addr: &str) -> Self {
        let address = Self::addr_from_str(addr);
        Self { address }
    }
}

impl Default for Proxy {
    fn default() -> Self {
        Self::from_str("127.0.0.1:8080")
    }
}

#[cfg(test)]
mod tests {
    use super::Proxy;

    #[test]
    fn test_default() {
        let a = Proxy::default();
        let b = a.clone();
        assert_eq!(a, b)
    }
}