pub mod models;

pub mod schemas {

}

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

pub mod structures {

}

