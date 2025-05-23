use bitcoin::Block;

use super::random::block::{BlockParams, RandomBlock};

pub struct GenerateBlock {}

impl GenerateBlock {
    pub fn valid_random(params: BlockParams) -> Block {
        Block::random(params)
    }
}
