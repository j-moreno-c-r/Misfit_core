use bitcoin::{block::Header, Block, Transaction};

use crate::transaction::{generator::GenerateTx, random::transaction::TxParams};

use super::header::{HeaderParams, RandomHeader};

pub struct BlockParams {
    pub header: Option<Header>,
    pub txs: Option<Vec<Transaction>>,
}

impl Default for BlockParams {
    fn default() -> Self {
        BlockParams {
            header: None,
            txs: None,
        }
    }
}

pub trait RandomBlock {
    fn random(params: BlockParams) -> Block;
}

impl RandomBlock for Block {
    // Implement params
    fn random(params: BlockParams) -> Block {
        Block {
            header: params
                .header
                .unwrap_or_else(|| Header::random(HeaderParams::default())),
            txdata: params
                .txs
                .unwrap_or_else(|| vec![GenerateTx::valid_random(TxParams::default())]),
        }
    }
}
