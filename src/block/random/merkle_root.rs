use bitcoin::{merkle_tree, Transaction, TxMerkleNode};
use secp256k1::rand::{self, Rng};

use crate::transaction::{generator::GenerateTx, random::transaction::TxParams};

#[derive(Default)]
pub struct MerkleRootParams {
    pub txs: Option<Vec<Transaction>>,
}

pub trait MerkleRoot {
    fn from_transactions(txs: Vec<Transaction>) -> TxMerkleNode;
    fn random(params: MerkleRootParams) -> TxMerkleNode;
}

impl MerkleRoot for TxMerkleNode {
    fn from_transactions(txs: Vec<Transaction>) -> TxMerkleNode {
        let hashes = txs.iter().map(|tx| tx.compute_txid().to_raw_hash());
        merkle_tree::calculate_root(hashes)
            .map(|h| h.into())
            .unwrap()
    }

    fn random(params: MerkleRootParams) -> TxMerkleNode {
        let txs = params.txs.unwrap_or_else(|| {
            let random = rand::thread_rng().gen_range(0..10);

            let mut txs = vec![];
            for _ in 0..random {
                txs.push(GenerateTx::valid_random(TxParams::default()));
            }

            txs
        });

        Self::from_transactions(txs)
    }
}
