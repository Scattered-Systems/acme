pub use crate::models::{tokens::*, users::*};

mod tokens;
mod users;

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
