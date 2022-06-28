/*
    Appellation: validators
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
use crate::{DIFFICULTY_PREFIX, Block, calculate_block_hash, compute_hash_binary_repr};

pub fn determine_block_validity(block: &Block, previous_block: &Block) -> bool {
    if block.previous != previous_block.hash {
        log::warn!("block with id: {} has wrong previous hash", block.id);
        return false;
    } else if !compute_hash_binary_repr(
        &hex::decode(&block.hash).expect("can decode from hex"),
    )
        .starts_with(DIFFICULTY_PREFIX)
    {
        log::warn!("block with id: {} has invalid difficulty", block.id);
        return false;
    } else if block.id != previous_block.id + 1 {
        log::warn!(
            "block with id: {} is not the next block after the latest: {}",
            block.id, previous_block.id
        );
        return false;
    } else if hex::encode(
        calculate_block_hash(
            block.data.clone(),
            block.id,
            block.nonce,
            block.previous.clone(),
            block.timestamp.clone(),
        )
    ) != block.hash
    {
        log::warn!("block with id: {} has invalid hash", block.id);
        return false;
    }
    true
}

pub fn determine_chain_validity(chain: &[Block]) -> bool {
    for i in 0..chain.len() {
        if i == 0 {
            continue;
        }
        let first = chain.get(i - 1).expect("has to exist");
        let second = chain.get(i).expect("has to exist");
        if !determine_block_validity(second, first) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}