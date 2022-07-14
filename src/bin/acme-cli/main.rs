/*
   Appellation: acme-cli
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use app::*;

mod app;

fn main() -> Result<(), acme::StandardError> {
    let app = Interface::new(false, "acme".to_string());
    app.run().ok().unwrap();
    Ok(())
}
