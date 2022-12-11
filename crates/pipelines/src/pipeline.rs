/*
    Appellation: pipeline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        PipelineStage:
            Stages in the build process which specify when a particular hook will execute
*/
use serde::{Deserialize, Serialize};

#[allow(clippy::enum_variant_names)]
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStage {
    PreBuild,
    #[default]
    Build,
    PostBuild,
}
