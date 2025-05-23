use bitcoin::TxMerkleNode;

use crate::transaction::{generator::GenerateTx, random::transaction::TxParams};

pub trait RandomMerkleRoot {
    fn random() -> TxMerkleNode;
}

impl RandomMerkleRoot for TxMerkleNode {
    // TODO: Try get from params
    fn random() -> TxMerkleNode {
        let tx_id = GenerateTx::valid_random(TxParams::default()).compute_txid();

        // !!TODO: GET TRANSACTIONS FROM PARAMS
        TxMerkleNode::from(tx_id)
    }
}
