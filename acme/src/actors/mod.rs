/*
    Module: Actors
    Overview:
        Definition: Actor
            In computer programming, actors are defined as mathematical models of computation.
        Goals:
            - Generally describe the fundamental actors that are being leveraged throughout
 */

pub use chains::*;
pub use loggers::*;

mod chains;
mod loggers;

pub trait Actor {
    type Appellation;
    // Defining the actor's name
    type Conduct;
    // Defining the actor's behaviour
    type Configuration;
    // Defining a new method of identifying an external actor
    type Data; // Define the standard format of data for the actor

    fn activate(appellation: Self::Appellation, configuration: Self::Configuration) -> Self;
}

