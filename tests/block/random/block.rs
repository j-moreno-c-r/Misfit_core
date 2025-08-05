use bitcoin::{block::Header, Block, Transaction};
use misfit_core::{
    block::random::{
        block::{BlockParams, RandomBlock},
        header::{HeaderParams, RandomHeader},
    },
    transaction::random::transaction::{RandomTransacion, TxParams},
};
use secp256k1::rand::{self, Rng};

#[test]
fn test_random_block() {
    let (block, height): (Block, u32) = Block::random(BlockParams::default());

    assert!(block.check_merkle_root());
    assert!(block.txdata.iter().count() >= 1);
    assert!(height >= 1);
}

#[test]
fn test_random_block_with_specified_header() {
    let expected_header: Header = Header::random(HeaderParams::default());

    let (block, _): (Block, u32) = Block::random(BlockParams {
        header: Some(expected_header),
        ..Default::default()
    });

    assert!(block.header.eq(&expected_header));
}

#[test]
fn test_random_block_with_specified_txs() {
    let expected_txs: Vec<Transaction> = vec![Transaction::random(TxParams::default())];

    let (block, _): (Block, u32) = Block::random(BlockParams {
        txs: Some(expected_txs.clone()),
        ..Default::default()
    });

    assert!(block.txdata.eq(&expected_txs));
}

#[test]
fn test_random_block_with_specified_height() {
    let expected_height: u32 = rand::thread_rng().gen::<u32>();

    let (_, height): (Block, u32) = Block::random(BlockParams {
        height: Some(expected_height),
        ..Default::default()
    });

    assert!(height.eq(&expected_height));
}
