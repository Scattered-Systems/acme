/*
    Appellation: router <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Router {
    pub path: String
}

impl Router {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}