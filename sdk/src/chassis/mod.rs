/*
   Appellation: Chassis
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       This module implements the chassis for creating EVM native SideChains

*/
pub use crate::chassis::projects::*;

mod projects;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ChassisStates {
    Connect { endpoint: String },
    Scaffold { name: String },
}

pub struct Chassis {
    state: ChassisStates,
}

impl std::fmt::Display for Chassis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "", )
    }
}
