use bitcoin::Block;
use super::random::block::{BlockParams, RandomBlock};
use bitcoin::OutPoint;

use crate::transaction::{
    generator::GenerateTx,
    random::{
        input::InputParams,
        transaction::{TxParams},
    },
};

pub struct GenerateBlock {}

impl GenerateBlock {
    pub fn valid_random(mut params: BlockParams) -> (Block, u32) {
    let input_params = InputParams {
        outpoint: Some(OutPoint::null()),
        ..Default::default()
    };

    let coinbase_params = TxParams {
        input: Some(input_params),
        ..Default::default()
    };

    let coinbase_info = GenerateTx::valid_random(coinbase_params);

    let mut txs = params.txs.take().unwrap_or_default();
    txs.insert(0, coinbase_info);
    params.txs = Some(txs);

    Block::random(params)   
}
}