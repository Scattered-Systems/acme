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
    pub alias: String,
    pub key: String,
    pub secret: String,
    pub token: String,
}

impl Account {
    pub fn new(address: String, alias: String, key: String, secret: String, token: String) -> Self {
        Self {
            address,
            alias,
            key,
            secret,
            token,
        }
    }
    pub fn from(address: &str, alias: &str, key: &str, secret: &str, token: &str) -> Self {
        Self::new(
            String::from(address),
            String::from(alias),
            String::from(key),
            String::from(secret),
            String::from(token),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
