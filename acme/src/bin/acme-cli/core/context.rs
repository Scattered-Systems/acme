/*
   Appellation: context
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AppStates {
    Start(String),
    Terminate(String),
    Transition(String),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Contexts {
    Application {
        address: String,
        mode: String,
        name: String,
        state: AppStates,
    },
}
