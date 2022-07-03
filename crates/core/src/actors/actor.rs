/*
   Appellation: actor
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ...Summary...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Actor {
    pub data: Vec<u8>,
}
