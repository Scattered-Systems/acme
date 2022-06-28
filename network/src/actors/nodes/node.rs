
#[derive(Clone, Debug)]
pub struct Node {
    pub peer: crate::Peer,
}

impl Node {
    pub fn new() -> Self {
        Self {
            peer: crate::Peer::new()
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node(peer(id={}))", self.peer.id)
    }
}