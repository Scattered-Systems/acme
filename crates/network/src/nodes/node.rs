use crate::Peer;

pub enum Chassis {
    Node { network: String },
}

#[derive(Clone, Debug)]
pub struct Node {
    pub client: Peer,
}

impl Node {
    pub fn new() -> Self {
        todo!()
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node()",)
    }
}
