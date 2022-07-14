/*
   Appellation: client
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

pub const CB_ENDPOINT: &str = "https://api.coinbase.com/api/v3";

pub type CBClientError = Box<dyn std::error::Error>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CoinbaseCredentials {
    CBPApiToken {
        key: String,
        passphrase: String,
        secret: String,
    },
    CBOAuth {
        access_type: String,
        token: String,
    },
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CoinbaseClient {
    pub credentials: CoinbaseCredentials,
    pub endpoint: String,
}

impl CoinbaseClient {
    fn authorize() {
        todo!()
    }
    fn connect() {
        todo!()
    }
    fn create(credentials: CoinbaseCredentials, endpoint: String) -> Result<Self, CBClientError> {
        Ok(Self {
            credentials,
            endpoint,
        })
    }
    fn destroy() {
        todo!()
    }

    pub fn new(credentials: CoinbaseCredentials, endpoint: String) -> Self {
        Self::create(credentials, endpoint).ok().unwrap()
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
