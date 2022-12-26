/*
   Appellation: compiler <state>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use scsys::prelude::{fnl_remove, StatePack};
use serde::{Deserialize, Serialize};
use std::convert::From;
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Copy, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum CompilerStates {
    Idle = 0,
    Init = 1,
    Read = 2,
    Compile = 3,
    Write = 4,
    Complete = 5,
    Invalid = 6,
}

impl CompilerStates {
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

impl StatePack for CompilerStates {}

impl std::fmt::Display for CompilerStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            fnl_remove(serde_json::to_string(self).unwrap()).to_ascii_lowercase()
        )
    }
}

impl Default for CompilerStates {
    fn default() -> Self {
        Self::Idle
    }
}

impl From<i64> for CompilerStates {
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

impl From<CompilerStates> for i64 {
    fn from(data: CompilerStates) -> Self {
        match data {
            CompilerStates::Idle => 0,
            CompilerStates::Init => 1,
            CompilerStates::Read => 2,
            CompilerStates::Compile => 3,
            CompilerStates::Write => 4,
            CompilerStates::Complete => 5,
            CompilerStates::Invalid => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_state() {
        let a: i64 = CompilerStates::default().into();
        assert_eq!(a, 0i64);
        assert_eq!(
            CompilerStates::try_from("idle").ok().unwrap(),
            CompilerStates::from(a)
        )
    }
}
