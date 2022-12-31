/*
    Appellation: cli <specs>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{AsyncHandler, Handler};
use clap::{Parser, Subcommand};

pub trait Commands: Clone + Default + Subcommand {
    fn command(&self) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }
}

pub trait AsyncCommands: Commands + AsyncHandler {}

pub trait CliSpec: Parser {
    type Cmds: Commands;

    fn new() -> Self {
        Self::parse()
    }
    fn command(&self) -> Option<Self::Cmds>
    where
        Self: Sized;
}

pub trait CliSpecExt: CliSpec + Handler {}

pub trait AsyncCliSpec: CliSpec + AsyncHandler {}
