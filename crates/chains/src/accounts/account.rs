/*
   Appellation: account
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Account {
    pub address: String,
    pub username: String,
    pub password: String,
}

impl Account {
    pub fn constructor(address: String, username: String, password: String) -> Self {
        Self { address, username, password }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_account() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
