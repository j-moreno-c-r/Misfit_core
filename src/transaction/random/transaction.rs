use super::{
    input::{InputParams, RandomInput},
    locktime::RandomLockTime,
    output::{OutputParams, RandomOutput},
    version::RandomVersion,
};
use bitcoin::{absolute::LockTime, transaction::Version, Transaction, TxIn, TxOut};

#[derive(Default)]
pub struct TxParams {
    pub(crate) version: Option<Version>,
    pub(crate) lock_time: Option<LockTime>,
    pub(crate) input: Option<InputParams>,
    pub(crate) output: Option<OutputParams>,
}


pub trait RandomTransacion {
    fn random(params: TxParams) -> Transaction;
}

impl RandomTransacion for Transaction {
    fn random(params: TxParams) -> Transaction {
        let input_params = params.input.unwrap_or_default();
        let output_params = params.output.unwrap_or_default();

        Transaction {
            version: params.version.unwrap_or_else(Version::random),
            lock_time: params.lock_time.unwrap_or_else(LockTime::random),
            input: vec![TxIn::random(input_params)],
            output: vec![TxOut::random(output_params)],
        }
    }
}
