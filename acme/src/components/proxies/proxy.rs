/*
   Appellation: proxy <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Proxy;

impl Proxy {
    pub fn address_from(host: [u8; 4], port: u16) -> std::net::SocketAddr {
        std::net::SocketAddr::from((host, port))
    }
}
