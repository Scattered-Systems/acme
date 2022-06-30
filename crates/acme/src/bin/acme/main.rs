/*
    Appellation: acme-cli
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use acme::CLI;
pub use crate::{api::*, core::*, interface::*};

mod api;
mod core;
mod interface;

fn main() {
    let app = App::new(false, "acme".to_string());
    println!("{:#?}", &app.configure());
    println!("{:#?}", &app.call())
}

