use bitcoin::{merkle_tree::calculate_root, Transaction, TxMerkleNode};
use misfit_core::{
    block::random::merkle_root::{MerkleRoot, MerkleRootParams},
    transaction::random::transaction::{RandomTransacion, TxParams},
};
use secp256k1::rand::{self, Rng};

#[test]
fn test_merkle_root_from_one_transaction() {
    let transaction: Transaction = Transaction::random(TxParams::default());
    let expected_hash = transaction.compute_txid().to_raw_hash();

    let merkle_root: TxMerkleNode = TxMerkleNode::from_transactions(vec![transaction]);

    assert!(merkle_root.as_raw_hash().eq(&expected_hash))
}

#[test]
fn test_merkle_root_from_some_transactions() {
    let mut transactions: Vec<Transaction> = vec![];

    let tx_count = rand::thread_rng().gen_range(2..100);

    for _ in 0..tx_count {
        transactions.push(Transaction::random(TxParams {
            ..Default::default()
        }));
    }

    let hashes = transactions
        .iter()
        .map(|tx| tx.compute_txid().to_raw_hash());
    let expected_hash = calculate_root(hashes).map(|h| h.into()).unwrap();

    let merkle_root: TxMerkleNode = TxMerkleNode::from_transactions(transactions);

    assert!(merkle_root.as_raw_hash().eq(&expected_hash))
}

#[test]
fn test_merkle_root_random() {
    let transaction: Transaction = Transaction::random(TxParams::default());
    let expected_hash = transaction.compute_txid().to_raw_hash();

    let merkle_root: TxMerkleNode = TxMerkleNode::random(MerkleRootParams {
        txs: Some(vec![transaction]),
    });

    assert!(merkle_root.as_raw_hash().eq(&expected_hash))
}
