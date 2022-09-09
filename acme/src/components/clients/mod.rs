/*
    Appellation: clients <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

pub mod authenticated;
pub mod public;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method, RequestBuilder,
};

pub trait WebClient: Clone + std::fmt::Debug {
    fn client(&mut self) -> reqwest::Client {
        reqwest::Client::new()
    }
    fn endpoint(&self) -> String;

    fn extend_path(&self, path: &str) -> String {
        format!("{}/{}", self.endpoint(), path)
    }
    fn quick_headers(&self, data: Vec<(&'static str, &'static str)>) -> HeaderMap {
        HeaderMap::from_iter(
            data.iter()
                .map(|i| (HeaderName::from_static(i.0), HeaderValue::from_static(i.1)))
                .collect::<Vec<_>>(),
        )
    }
    fn new_request(&mut self, method: Method, path: Option<&str>) -> RequestBuilder {
        let url = match path {
            Some(v) => self.extend_path(v),
            None => self.endpoint(),
        };
        self.client().request(method, url.as_str())
    }
}
