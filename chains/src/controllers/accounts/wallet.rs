/*
   Appellation: wallet
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Wallet {
    assets: Vec<String>,
    network: String,
}

impl Wallet {
    pub fn constructor(assets: Vec<String>, network: String) -> Self {
        Self { assets, network }
    }
}

impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wallet(assets={:#?}, network={:#?})",
            self.assets, self.network
        )
    }
}
