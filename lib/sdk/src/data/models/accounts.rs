/*
   Appellation: accounts
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Accounts {
    Acct(Account),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Account {
    pub address: String,
}

impl Account {
    pub fn constructor(address: String) -> Self {
        Self { address }
    }
    pub fn new(address: String) -> Self {
        Self::constructor(address)
    }
    pub fn from<T>(address: &T) -> Self {
        match address.try_into() {
            Ok(v) => Self::constructor(v),
            Err(e) => panic!("{}", e),
        }
    }
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account(address={})", self.address)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
