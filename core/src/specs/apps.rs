/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::AsyncSpawable;
use scsys::prelude::{AsyncResult, Locked};

/// Implements the base interface for creating compatible platform applications
pub trait AppSpec: Default + AsyncSpawable {
    type Cnf;
    type Ctx;
    type State;
    fn init() -> Self;
    fn context(&self) -> Self::Ctx;
    fn name(&self) -> String;
    fn settings(&self) -> Self::Cnf;
    fn setup(&mut self) -> AsyncResult<&Self>;
    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
    fn state(&self) -> &Locked<Self::State>;
}
