use crate::chains::block::Block;

#[derive(Clone, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub operator: String,
}

impl Blockchain {
    pub fn new(operator: String) -> Self {
        let chain = vec![];
        Self {
            chain,
            operator,
        }
    }
}                        