mod appellation;

pub use appellation::*;

mod controller {
    pub trait ControllerSpec {
        type Actor;
        type Context;
    }
}