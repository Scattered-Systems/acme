pub use crate::models::{accounts::*, tokens::*, users::*, wallet::*};

mod accounts;
mod tokens;
mod users;
mod wallet;

pub trait AsyncModel {
    type Actor;
    type Client;
    type Config;
    type Data;

    fn controller(config: Self::Config);
    fn constructor(&self) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

pub trait StandardModel {
    type Actor;
    type Client;
    type Config;
    type Data;
}
