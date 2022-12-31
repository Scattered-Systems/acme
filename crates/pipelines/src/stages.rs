/*
    Appellation: pipeline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Stages
            -
*/
use serde::{Deserialize, Serialize};

#[allow(clippy::enum_variant_names)]
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStage {
    PreBuild = 0,
    #[default]
    Build = 1,

    PostBuild = -1,
}
