pub const MERKLE_LEAF_SIG: u8 = 0u8;
pub const MERKLE_INTERNAL_SIG: u8 = 1u8;

pub type MerkleHash = Vec<u8>;

pub struct MerkleNode;

pub struct MerkleLeaf;

pub struct MerkleTree {
    pub leaves: Vec<MerkleLeaf>,
    pub nodes: MerkleNode,
}
