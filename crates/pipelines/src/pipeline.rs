/*
    Appellation: pipeline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        PipelineStage:
            Stages in the build process which specify when a particular hook will execute
*/
use crate::PipelineStage;

use scsys::prelude::{Message, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Pipeline {
    pub message: Message,
    pub name: String,
    pub stage: PipelineStage,
    pub timestamp: i64,
}

impl Pipeline {
    pub fn new(message: Option<Message>, name: String) -> Self {
        let message = message.unwrap_or_default();
        let stage = PipelineStage::PreBuild;
        let timestamp = Timestamp::default().into();
        Self {
            message,
            name,
            stage,
            timestamp,
        }
    }
}
