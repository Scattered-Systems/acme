/*
   Appellation: error
   Context:
   Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
   Description:
       ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ErrorFrame {
    pub state: String,
}
