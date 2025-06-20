use super::{
    input::{InputParams, RandomInput},
    locktime::RandomLockTime,
    output::{OutputParams, RandomOutput},
    version::RandomVersion,
};
use bitcoin::{
    absolute::LockTime, transaction::Version, NetworkKind, PrivateKey, Transaction, TxIn, TxOut,
};

pub struct TxParams {
    pub(crate) version: Option<Version>,
    pub(crate) lock_time: Option<LockTime>,
    pub(crate) input: Option<InputParams>,
    pub(crate) output: Option<OutputParams>,
    pub(crate) private_key: Option<PrivateKey>,
}

impl Default for TxParams {
    fn default() -> Self {
        TxParams {
            version: None,
            lock_time: None,
            input: None,
            output: None,
            private_key: None,
        }
    }
}

pub trait RandomTransacion {
    fn random(params: TxParams) -> Transaction;
}

impl RandomTransacion for Transaction {
    fn random(params: TxParams) -> Transaction {
        let private_key = params
            .private_key
            .unwrap_or_else(|| PrivateKey::generate(NetworkKind::Main));

        let mut input_params = params.input.unwrap_or_default();
        let mut output_params = params.output.unwrap_or_default();

        input_params.private_key = Some(private_key);
        output_params.private_key = Some(private_key);

        // Gerar input e output com suas informações
        let input_info = TxIn::random(input_params);
        let output_info = TxOut::random(output_params);

        let transaction = Transaction {
            version: params.version.unwrap_or_else(|| Version::random()),
            lock_time: params.lock_time.unwrap_or_else(|| LockTime::random()),
            input: vec![input_info.clone()],
            output: vec![output_info.0.clone()],
        };

        transaction
    }
}
