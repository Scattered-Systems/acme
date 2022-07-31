/*
    Appellation: create <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CrudState {
    Create,
    Read,
    Update,
    Delete,
}

impl Default for CrudState {
    fn default() -> Self {
        Self::Read
    }
}
