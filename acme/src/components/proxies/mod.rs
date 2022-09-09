/*
    Appellation: proxies <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub mod proxy;

use std::net::SocketAddr;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProxyParam {
    Dest(std::net::SocketAddr),
    Link(SocketAddr),
}

