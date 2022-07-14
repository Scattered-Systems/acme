/*
   Appellation: acme-cli
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use app::*;

mod app;

fn main() {
    let app = Interface::new(false, "acme".to_string());
    println!("{:#?}", &app.run())
}