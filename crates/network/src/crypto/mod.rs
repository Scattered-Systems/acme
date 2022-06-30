/*
    Appellation: Contracts
    Context: Module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
 */
mod contract;

pub use contract::*;


pub enum ContractStates {
    Aborted,
    Executed,
    Uploaded,
}

pub trait Contract {
    type Behaviour;
    type Chain;
    type Container;
    type Data;

    fn constructor() -> Self;
}