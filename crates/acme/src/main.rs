/*
    Appellation: acme-cli
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Initial efforts are being placed towards designing a project management suite of utilities
        for building scalable, user-centric dApps.
 */

use acme::application::{Application, CLI};

fn main() {
    let args = Application::commands(&Application);

    for _ in 0..args.count {
        println!("Hello {}!", args.appellation)
    }
}