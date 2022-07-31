/*
   Appellation: primitives <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use constants::*;
pub use types::*;

mod constants {
    ///
    pub const STANDARD_OAUTH_TOKEN_PATH: &str = "oauth/token";
}

mod types {
    pub use async_trait::async_trait;
    use axum::routing::IntoMakeService;
    pub use hyper::server::conn::AddrIncoming;

    /// Type alias for the servers implemented when leveraging the axum web framework
    pub type AxumServer =
    axum::Server<hyper::server::conn::AddrIncoming, IntoMakeService<axum::Router>>;
    /// Type alias for the host when creating [std::net::SocketAddr] from pieces
    pub type HostPiece = [u8; 4];
    /// Type alias for the port when creating [std::net::SocketAddr] from pieces
    pub type PortPiece = u16;
    /// Type alias for the pieces when creating [std::net::SocketAddr] from pieces
    pub type SocketAddrPieces = (HostPiece, PortPiece);

    /// Describes the different potential host options, with an extraction tool for ease of use
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum ServerHost {
        Std(HostPiece),
        Str(String),
    }

    impl ServerHost {
        pub fn extract(data: String) -> Self {
            Self::Std(
                scsys::extract::Extractor::new('.', data)
                    .extract()
                    .try_into()
                    .ok()
                    .unwrap(),
            )
        }
    }

    impl Default for ServerHost {
        fn default() -> Self {
            Self::Std([0, 0, 0, 0])
        }
    }
}
