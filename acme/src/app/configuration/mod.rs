/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
pub use crate::app::configuration::{
    account::*,
    appellation::*,
};

mod account;
mod appellation;

pub enum Settings {
    Application,
}