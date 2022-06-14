pub use super::specs::Node as NodeSpec;

#[derive(Clone, Debug)]
pub struct Node;

impl NodeSpec for Node {
    type Configuration = ();

    fn new() -> Self {
        todo!()
    }

    fn setup(&self) -> Self {
        todo!()
    }
}