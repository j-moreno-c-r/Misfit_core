use super::random::transaction::{RandomTransacion, TxParams};
use bitcoin::Transaction;
use bitcoin::{
    key::{Secp256k1},
    NetworkKind, PrivateKey
};

pub struct GenerateTx {}

impl GenerateTx {
    pub fn valid_random(params: TxParams) -> Transaction {
        let curve = Secp256k1::new();
        let privatekey = PrivateKey::generate(NetworkKind::Main);
        Transaction::random(params, &curve, &privatekey)
    }
}