/*
   Appellation: acme-api
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::{app::*, core::*};

mod app;
mod core;

fn main() {
    let app = Interface::new(false, "acme".to_string());
    println!("{:#?}", &app.run())
}
