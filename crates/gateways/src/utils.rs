/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Gateway;
use s3::{creds::Credentials, serde_types::ListBucketResult, Region};
use scsys::prelude::{BoxResult, S3Credential};

///
pub fn convert_credentials(cred: S3Credential) -> Credentials {
    Credentials {
        access_key: Some(cred.access_key),
        secret_key: Some(cred.secret_key),
        security_token: None,
        session_token: None,
        expiration: None,
    }
}
///
pub fn simple_creds(access_key: &str, secret_key: &str) -> Credentials {
    Credentials::new(Some(access_key), Some(secret_key), None, None, None).expect("msg")
}
///
pub fn simple_region<T: std::string::ToString>(endpoint: T, region: T) -> Region {
    Region::Custom {
        endpoint: endpoint.to_string(),
        region: region.to_string(),
    }
}

///
pub async fn collect_obj_names(objects: Vec<ListBucketResult>) -> Vec<String> {
    tracing::info!("Collecting information on the given data...");
    objects
        .iter()
        .map(|i| i.clone().name)
        .collect::<Vec<String>>()
}
///
pub async fn fetch_bucket_contents(
    bucket: s3::Bucket,
    prefix: &str,
    delim: Option<String>,
) -> BoxResult<Vec<ListBucketResult>> {
    let res = bucket.list(prefix.to_string(), delim).await?;
    Ok(res)
}
///
pub async fn fetch_bucket(gateway: &Gateway, name: &str) -> BoxResult<s3::Bucket> {
    Ok(gateway.bucket(name)?)
}
