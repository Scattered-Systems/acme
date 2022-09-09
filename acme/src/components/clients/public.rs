/*
    Appellation: public <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

use reqwest::{header::{HeaderMap, HeaderName, HeaderValue}, Client, Response};

#[derive(Clone, Debug)]
pub struct PubClient {
    client: Client,
    pub endpoint: String
}

impl PubClient {
    pub fn new(endpoint: String) -> Self {
        let client = Client::new();

        Self { client, endpoint }
    }
    pub fn extend_path(&self, path: &str) -> String {
        format!("{}/{}", self.endpoint, path)
    }
    pub fn set_headers(&self, data: Vec<(&'static str, &'static str)>) -> HeaderMap {
        HeaderMap::from_iter(data.iter().map(|i| (HeaderName::from_static(i.0), HeaderValue::from_static(i.1))).collect::<Vec<_>>())
    }
    pub async fn fetch(&self, path: Option<&str>) -> scsys::BoxResult<Response> {
        let url = match path {
            Some(_) => format!("{}/{}", self.endpoint.clone(), path.expect("")),
            None => self.endpoint.clone()
        };
        let headers = vec![("Accepts", "application/json"), ("User-Agent", crate::ME_USER_AGENT)];
        let res = self.client
            .get(url.as_str())
            .headers(self.set_headers(headers))
            .send()
            .await?;
        Ok(res)
    }
    
}

#[cfg(test)]
mod tests {
    use super::PubClient;

    #[test]
    fn test_public_client() {
        let a = PubClient::new("https://google.com".to_string());
        let b = a.clone();
        assert_eq!(a.endpoint, b.endpoint)
    }
}