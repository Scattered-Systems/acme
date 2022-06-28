#[derive(Clone, Debug, serde::Deserialize, Hash, PartialEq, serde::Serialize)]
pub struct Account {
    ensname: String,
}
