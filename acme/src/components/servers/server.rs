/*
   Appellation: server <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use async_trait::async_trait;

/// Outline the available power-based states for servers
pub enum ServerState {
    Off,
    On,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::Off
    }
}

/// Outline a typical server object
#[async_trait]
pub trait IServer<As = ServerState> {
    fn address(&self, host: crate::HostPiece, port: crate::PortPiece) -> std::net::SocketAddr {
        std::net::SocketAddr::from((host, port))
    }
    async fn client(&self) -> Result<axum::Router, scsys::BoxError>
        where
            Self: Sized;
    async fn run(&mut self, host: crate::HostPiece, port: crate::PortPiece) -> crate::AxumServer
        where
            Self: Sized + Sync,
    {
        axum::Server::bind(&self.address(host, port))
            .serve(self.client().await.ok().unwrap().into_make_service())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Server {
    pub host: [u8; 4],
    pub port: u16,
}

impl Server {
    pub fn new(host: [u8; 4], port: u16) -> Self {
        Self { host, port }
    }
    pub fn extract_host_from(breakpoint: char, host: String) -> crate::HostPiece {
        scsys::extract::Extractor::new(breakpoint, host)
            .extract()
            .try_into()
            .ok()
            .unwrap()
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new([0, 0, 0, 0], 8080)
    }
}
