/*
   Appellation: compiler <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::states::{CompilerState, CompilerStates};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compiler {
    state: Arc<CompilerState>,
}

impl Compiler {
    pub fn set_state(&mut self, state: CompilerStates) -> &Self {
        self.state = Arc::new(CompilerState::new(None, None, Some(state)));
        self
    }
    pub fn init(&mut self) -> &Self {
        self.set_state(CompilerStates::init());
        self
    }

    pub fn read_input(&mut self) -> &Self {
        self.set_state(CompilerStates::read());
        // read the input WebAssembly code
        // ...
        // return the next state
        self
    }

    pub fn compile(&mut self) -> &Self {
        self.set_state(CompilerStates::compile());
        // compile the input WebAssembly code using Wasmer
        // ...
        // return the next state
        self
    }

    pub fn write_output(&mut self) -> &Self {
        self.set_state(CompilerStates::write());
        // write the compiled WebAssembly code to the output
        // ...
        // return the next state
        self
    }

    pub fn finish(&mut self) -> &Self {
        self.set_state(CompilerStates::complete());
        // clean up and finalize the compilation process
        // ...
        // return the final state
        self
    }

    pub fn run(&mut self) {
        // update the current state of the state-machine
        // by calling the appropriate state method based
        // on the current state
    }
}
