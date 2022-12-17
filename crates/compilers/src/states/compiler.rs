/*
   Appellation: compiler <state>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::convert::From;
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Copy, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum CompilerState {
    Idle = 0,
    Init = 1,
    Read = 2,
    Compile = 3,
    Write = 4,
    Complete = 5,
    Invalid = 6,
}

impl CompilerState {
    pub fn idle() -> Self {
        Self::Idle
    }
    pub fn init() -> Self {
        Self::Init
    }
    pub fn invalid() -> Self {
        Self::Invalid
    }
    pub fn read() -> Self {
        Self::Read
    }
    pub fn write() -> Self {
        Self::Write
    }
    pub fn compile() -> Self {
        Self::Compile
    }
    pub fn complete() -> Self {
        Self::Complete
    }
}

impl Default for CompilerState {
    fn default() -> Self {
        Self::Idle
    }
}

impl From<i64> for CompilerState {
    fn from(data: i64) -> Self {
        match data {
            0 => Self::idle(),
            1 => Self::init(),
            2 => Self::read(),
            3 => Self::compile(),
            4 => Self::write(),
            5 => Self::complete(),
            _ => Self::invalid(),
        }
    }
}

impl From<CompilerState> for i64 {
    fn from(data: CompilerState) -> Self {
        match data {
            CompilerState::Idle => 0,
            CompilerState::Init => 1,
            CompilerState::Read => 2,
            CompilerState::Compile => 3,
            CompilerState::Write => 4,
            CompilerState::Complete => 5,
            CompilerState::Invalid => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_state() {
        let a: i64 = CompilerState::default().into();
        assert_eq!(a, 0i64);
        assert_eq!(
            CompilerState::try_from("idle").ok().unwrap(),
            CompilerState::from(a)
        )
    }
}
