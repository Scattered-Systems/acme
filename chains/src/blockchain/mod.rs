/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
mod blocks;
mod chains;

pub use crate::blockchain::{blocks::*, chains::*};

pub type Blockchain = Vec<Block>;
