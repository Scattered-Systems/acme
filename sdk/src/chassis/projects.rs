/*
   Appellation: projects
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use utils::*;

/// Available projects with preconfigured pipelines
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Scaffolding {
    Application,
    Blockchain,
    Node,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProjectStages<T = String> {
    Development(T),
    Staging(T),
    Testing(T),
    Production(T),
}

/// A standard interface for managing projects
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Project {
    appellation: String,
    stage: ProjectStages,
}

impl Project {
    pub fn constructor(appellation: String, stage: ProjectStages) -> Self {
        Self { appellation, stage }
    }

    pub fn new(appellation: String, stage: ProjectStages) -> Self {
        Self::constructor(appellation, stage)
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "", )
    }
}

mod utils {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
