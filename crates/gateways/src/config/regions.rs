/*
    Appellation: regions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use s3::Region;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct S3Region {
    pub endpoint: String,
    pub region: String,
}

impl S3Region {
    pub fn new(endpoint: String, region: String) -> Self {
        Self { endpoint, region }
    }
    pub fn endpoint(&self) -> String {
        self.endpoint.clone()
    }
    pub fn region(&self) -> String {
        self.region.clone()
    }
    pub fn update_endpoint(&mut self, endpoint: String) -> &Self {
        self.endpoint = endpoint;
        self
    }
    pub fn update_region(&mut self, region: String) -> &Self {
        self.region = region;
        self
    }
}

impl Default for S3Region {
    fn default() -> Self {
        Self::new(
            "https://gateway.storjshare.io".to_string(),
            "us-east-1".to_string(),
        )
    }
}

impl std::fmt::Display for S3Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl std::convert::From<(&str, &str)> for S3Region {
    fn from(data: (&str, &str)) -> Self {
        Self::new(data.0.to_string(), data.1.to_string())
    }
}
impl std::convert::From<&Region> for S3Region {
    fn from(data: &Region) -> Self {
        Self::new(data.clone().endpoint(), data.clone().host())
    }
}

impl std::convert::From<&S3Region> for Region {
    fn from(data: &S3Region) -> Self {
        Self::Custom {
            endpoint: data.clone().endpoint,
            region: data.clone().region,
        }
    }
}
