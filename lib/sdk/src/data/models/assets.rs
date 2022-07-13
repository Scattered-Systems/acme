/*
    Appellation: assets
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */

pub trait FungibleTokenSpec<Cnt = String, Dt = usize> {
    fn fetch_symbol(&self) -> Cnt where Self: Sized;
    fn fetch_supply(&self) -> Dt where Self: Sized;
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AssetClass {
    Fungible,
    NonFungible,
    Standard(Asset),
}

pub struct Asset {
    pub data: Vec<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}