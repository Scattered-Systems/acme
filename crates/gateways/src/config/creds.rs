/*
    Appellation: creds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use s3::creds::Credentials;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GatewayCreds {
    pub access_key: String,
    pub(crate) secret_key: String,
}

impl GatewayCreds {
    pub fn new(access_key: String, secret_key: String) -> Self {
        Self {
            access_key,
            secret_key,
        }
    }
    pub fn from_env(
        &mut self,
        access_key: Option<&str>,
        secret_key: Option<&str>,
    ) -> scsys::BoxResult<&Self> {
        self.access_key = std::env::var(access_key.unwrap_or("S3_ACCESS_KEY"))?;
        self.secret_key = std::env::var(secret_key.unwrap_or("S3_SECRET_KEY"))?;
        Ok(self)
    }
}

impl std::fmt::Display for GatewayCreds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!({"access_key": &self.access_key}))
    }
}

impl std::convert::TryFrom<Credentials> for GatewayCreds {
    type Error = Box<dyn std::error::Error>;
    fn try_from(data: Credentials) -> Result<Self, Self::Error> {
        if data.access_key.is_some() & data.secret_key.is_some() {
            let res = Self::new(data.access_key.unwrap(), data.secret_key.unwrap());
            Ok(res)
        } else {
            panic!("Failed to find any credentials")
        }
    }
}

impl std::convert::TryFrom<GatewayCreds> for Credentials {
    type Error = s3::creds::error::CredentialsError;
    fn try_from(data: GatewayCreds) -> Result<Self, Self::Error> {
        Credentials::new(
            Some(data.access_key.as_str()),
            Some(data.secret_key.as_str()),
            None,
            None,
            None,
        )
    }
}
