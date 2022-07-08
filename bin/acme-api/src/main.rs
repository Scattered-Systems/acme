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

use acme;

#[tokio::main]
async fn main() -> Result<(), AsyncError> {
    acme::get_cb_time();
    let mut interface = Interface::new();
    println!("{}", &interface);
    interface.run().await;
    Ok(())
}
