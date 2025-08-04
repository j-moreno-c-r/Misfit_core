use bitcoin::Transaction;
use misfit_core::block::generator::GenerateBlock;
use misfit_core::block::random::block::BlockParams;
use misfit_core::transaction::random::transaction::{RandomTransacion, TxParams};
use secp256k1::rand::{self, Rng};

#[test]
fn test_generate_block_valid_random() {
    let (block, _) = GenerateBlock::valid_random(BlockParams {
        ..Default::default()
    });

    assert!(block.check_merkle_root());
    assert!(block.txdata.iter().count() == 1);
}

#[test]
fn test_generate_block_valid_random_with_one_transaction() {
    let transactions = vec![Transaction::random(TxParams {
        ..Default::default()
    })];

    let (block, _) = GenerateBlock::valid_random(BlockParams {
        txs: Some(transactions.clone()),
        ..Default::default()
    });

    assert!(block.check_merkle_root());
    assert!(block.txdata.iter().count() == 2);
    assert!(block.txdata[1] == transactions[0]);
}

#[test]
fn test_generate_block_valid_random_with_some_transactions() {
    let mut transactions: Vec<Transaction> = vec![];

    let tx_count = rand::thread_rng().gen_range(2..100);

    for _ in 0..tx_count {
        transactions.push(Transaction::random(TxParams {
            ..Default::default()
        }));
    }

    let (block, _) = GenerateBlock::valid_random(BlockParams {
        txs: Some(transactions.clone()),
        ..Default::default()
    });

    assert!(block.check_merkle_root());
    assert!(block.txdata.iter().count() == (tx_count + 1));
    assert!(block.txdata[1..] == transactions.clone());
}
