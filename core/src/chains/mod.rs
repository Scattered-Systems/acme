/*
    Module: blockchain
    Overview:
        Scaffold the framework for the implementation of highly optimized, zk-Knowledge blockchains
        that remain EVM compatible
 */
pub use crate::chains::{
    blockchain::*,
    blocks::*,
};

pub(crate) mod blockchain;
pub(crate) mod blocks;

