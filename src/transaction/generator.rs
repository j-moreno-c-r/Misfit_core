use super::random::transaction::{RandomTransacion, TxParams};
use bitcoin::Transaction;

pub struct GenerateTx {}

impl GenerateTx {
    pub fn valid_random(params: TxParams) -> Transaction {
        Transaction::random(params)
    }
}