/*
    Appellation: public <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use openidconnect::url::Url;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};

#[derive(Clone, Debug)]
pub struct Public {
    pub endpoint: String,
    pub headers: HeaderMap,
}

impl Public {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            headers: HeaderMap::new(),
        }
    }

    pub fn client(&self) -> Client {
        Client::new()
    }

    pub fn set_headers(&mut self, data: Vec<(&'static str, &'static str)>) {
        for i in data {
            self.headers.append(i.0, HeaderValue::from_static(i.1));
        }
    }

    pub async fn fetch(&self, path: Option<&str>) -> Response {
        self.client()
            .get(self.url(path))
            .headers(self.headers.clone())
            .send()
            .await
            .expect("")
    }

    pub async fn fetch_json(&self, path: Option<&str>) -> serde_json::Value {
        self.fetch(path).await.json().await.expect("Response Error")
    }
    pub fn url(&self, path: Option<&str>) -> Url {
        let data = match path {
            Some(v) => format!("{}{}", self.endpoint, v),
            None => self.endpoint.clone(),
        };
        reqwest::Url::parse(data.as_str()).expect("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_public_client() {
        let mut client = Public::new("https://api.exchange.coinbase.com".to_string());
        client.set_headers(vec![
            ("Accept", "application/json"),
            ("User-Agent", crate::ME_USER_AGENT),
        ]);
        let a = client.fetch_json(Some("/currencies")).await;
        let b = a.clone();
        assert_eq!(a, b)
    }
}
