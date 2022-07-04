/*
   Appellation: acme-api
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::{endpoints::*, interface::*};

mod endpoints;
mod interface;

type AsyncStdError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), AsyncStdError> {
    Ok(())
}
