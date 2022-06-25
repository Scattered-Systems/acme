/*
    Appellation: acme-cli
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Initial efforts are being placed towards designing a project management suite of utilities
        for building scalable, user-centric dApps.
 */
pub use crate::{application::*, commands::*, controller::*};

mod application;
mod commands;
mod controller;

fn main() -> Result<(), acme::BoxedError> {
    let app = Application::new(Appellation::from("acme"));
    let args = app.constructor();

    println!("Welcome to acme {}", args.appellation);
    Ok(())
}