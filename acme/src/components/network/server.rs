/*
    Appellation: servers <module>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StandardServer {
    pub host: [u8; 4],
    pub port: u16,
}

impl StandardServer {
    pub fn new(host: [u8; 4], port: u16) -> Self {
        Self { host, port }
    }
    pub fn address_from(&mut self) -> std::net::SocketAddr {
        std::net::SocketAddr::from((self.host, self.port))
    }
}
