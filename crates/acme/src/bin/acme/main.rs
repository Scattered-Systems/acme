/*
   Appellation: acme
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::{core::*, interface::*};

mod core;
mod interface;

fn main() {
    let app = App::new(false, "acme".to_string());
    println!("{:#?}", &app.call())
}
