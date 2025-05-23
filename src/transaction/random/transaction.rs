use super::{
    input::{InputParams, RandomInput},
    locktime::RandomLockTime,
    output::{OutputParams, RandomOutput},
    version::RandomVersion,
};
use bitcoin::{absolute::LockTime, transaction::Version, Transaction, TxIn, TxOut};

pub struct TxParams {
    pub version: Option<Version>,
    pub lock_time: Option<LockTime>,
    pub input_count: Option<usize>,
    pub input: Option<InputParams>,
    pub output_count: Option<usize>,
    pub output: Option<OutputParams>,
}

impl Default for TxParams {
    fn default() -> Self {
        TxParams {
            version: None,
            lock_time: None,
            input_count: Some(1),
            input: None,
            output: None,
            output_count: Some(1),
        }
    }
}

pub trait RandomTransacion {
    fn random(params: TxParams) -> Transaction;
}

impl RandomTransacion for Transaction {
    fn random(params: TxParams) -> Transaction {
        let input_params = params.input.unwrap_or_default();
        let output_params = params.output.unwrap_or_default();

        let mut inputs = vec![];
        for _ in 0..params.input_count.unwrap_or(1) {
            inputs.push(TxIn::random(input_params.clone()));
        }

        let mut outputs = vec![];
        for _ in 0..params.input_count.unwrap_or(1) {
            outputs.push(TxOut::random(output_params.clone()));
        }

        Transaction {
            version: params.version.unwrap_or_else(|| Version::random()),
            lock_time: params.lock_time.unwrap_or_else(|| LockTime::random()),
            input: inputs,
            output: outputs,
        }
    }
}
