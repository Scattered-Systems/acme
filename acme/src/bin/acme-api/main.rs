/*
   Appellation: maximus
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::{api::*, clients::*, core::*, data::*};

mod api;
mod clients;
mod core;
mod data;

pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), AsyncError> {
    Interface::new().await;

    Ok(())
}
