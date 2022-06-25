/*
    Appellation: chain
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

use serde::{Deserialize, Serialize};

use crate::Block;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blockchain {
    pub appellation: String,
    pub configuration: String,
    pub context: String,
    pub data: Vec<Block>,
}