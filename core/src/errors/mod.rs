/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
mod error;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;
