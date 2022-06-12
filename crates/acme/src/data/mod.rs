pub mod models;
pub mod proofs;
pub mod schemas;
pub mod structures;

pub mod states {
    pub enum BlockStates {
        Invalid,
        Valid,
        Validating
    }

    pub enum ChainStates {
        Initializing,
        Initialized,
        Terminating,
        Terminated
    }
}



