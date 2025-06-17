use bitcoin::{block::Header, Block, Transaction};
use secp256k1::rand::{self, Rng};

use crate::transaction::{
    generator::GenerateTx, 
    random::transaction::{TxParams}
};

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
    fn random(params: BlockParams) -> Block {
        let tx_data = params.txs.unwrap_or_else(|| {
            let random = rand::thread_rng().gen_range(0..10);

            let mut txs = vec![];
            for _ in 0..random {
                let tx_info = GenerateTx::valid_random(TxParams::default());
                txs.push(tx_info);
            }

            txs
        });

        let header = params.header.unwrap_or_else(|| {
            let mut header_params = HeaderParams::default();
            header_params.txs = Some(tx_data.clone());

            Header::random(header_params)
        });

        Block {
            header,
            txdata: tx_data,
        }
    }
}