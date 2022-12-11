/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::config::GatewayConfig;
use s3::{error::S3Error, Bucket};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub cnf: GatewayConfig,
}

impl Context {
    pub fn new(cnf: GatewayConfig) -> Self {
        Self { cnf }
    }
    pub fn credentials(&self) -> s3::creds::Credentials {
        self.cnf.credentials()
    }
    pub fn region(&self) -> s3::Region {
        self.cnf.region()
    }
    pub fn bucket(&self, name: &str) -> Result<Bucket, S3Error> {
        Bucket::new(name, self.region(), self.credentials())
    }
}
