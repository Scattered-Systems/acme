/*
   Appellation: Chassis
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       This module implements the chassis for creating EVM native SideChains

*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ChassisStates {
    Connect { endpoint: String },
    Scaffold { name: String },
}

pub struct Chassis {
    state: ChassisStates,
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
