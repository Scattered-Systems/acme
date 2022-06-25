/*
    Appellation: context
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Context<C> {
    pub configuration: C,
}

impl Context<crate::Configuration> {
    pub fn new(configuration: crate::Configuration) -> Self {
        Self {
            configuration
        }
    }
}