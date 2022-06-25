/*
    Appellation: acme-cli
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Initial efforts are being placed towards designing a project management suite of utilities
        for building scalable, user-centric dApps.
 */
pub use crate::{app::*, controllers::*, core::*};

mod app;
mod controllers;
mod core;

fn get_arguments() {
    let args: Vec<_> = std::env::args().collect(); // get all arguments passed to app

    println!("{:?}", args);
}

fn main() -> Result<(), acme::BoxedError> {
    let app = Application::new("acme");
    let commands = app.constructor();
    let args = get_arguments();
    println!("Welcome to acme");
    println!("{:#?}", commands);
    Ok(())
}