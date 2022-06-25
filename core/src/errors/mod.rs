/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use std::error::Error;

pub use crate::errors::{error::*};

mod error;

pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;
