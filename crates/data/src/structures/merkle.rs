pub struct MerkleNode;

pub struct MerkleLeaf;

pub struct MerkleTree {
    pub leaves: Vec<MerkleLeaf>,
    pub nodes: MerkleNode,
}
