use super::{
    input::{InputParams, RandomInput},
    locktime::RandomLockTime,
    output::{OutputParams, RandomOutput},
    version::RandomVersion,
};
use bitcoin::{
    absolute::LockTime, transaction::Version, NetworkKind, PrivateKey, Transaction, TxIn, TxOut,ScriptBuf,consensus::Encodable,
};


#[derive(Default)]
pub struct TxParams {
    pub version: Option<Version>,
    pub lock_time: Option<LockTime>,
    pub input: Option<InputParams>,
    pub output: Option<OutputParams>,
    pub private_key: Option<PrivateKey>,
    pub block_height: Option<u32>,
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

        let mut input_info = TxIn::random(input_params);
        if let Some(height) = params.block_height {
            input_info.script_sig = prepend_bip34_height(input_info.script_sig, height);
        }
        let output_info = TxOut::random(output_params);

        Transaction {
            version: params.version.unwrap_or_else(Version::random),
            lock_time: params.lock_time.unwrap_or_else(LockTime::random),
            input: vec![input_info.clone()],
            output: vec![output_info.0.clone()],
        }
    }
}

fn prepend_bip34_height(script: ScriptBuf, height: u32) -> ScriptBuf {
    let mut height_bytes = vec![];
    bitcoin::consensus::encode::VarInt(height as u64).consensus_encode(&mut height_bytes).unwrap();
    let mut new_bytes = height_bytes;
    new_bytes.extend_from_slice(script.as_bytes());
    ScriptBuf::from_bytes(new_bytes)
}