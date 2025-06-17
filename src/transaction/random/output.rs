use bitcoin::{Amount, ScriptBuf, TxOut, PrivateKey, secp256k1::{All, Secp256k1}};
use secp256k1::rand::{self, Rng};

use super::script::{RandomScript, ScriptParams, ScriptTypes};

pub struct OutputParams {
    pub value: Option<Amount>,
    pub(crate) script_pubkey: Option<ScriptBuf>,
    pub script_params: Option<ScriptParams>,
}

pub struct OutputInfo {
    pub txout: TxOut,
    pub script_type: ScriptTypes,
}

impl Default for OutputParams {
    fn default() -> Self {
        OutputParams {
            value: None,
            script_pubkey: None,
            script_params: None,
        }
    }
}

pub trait RandomOutput {
    fn random(params: OutputParams, curve: &Secp256k1<All>, privatekey: &PrivateKey) -> (TxOut, ScriptTypes);
}

impl RandomOutput for TxOut {
    fn random(params: OutputParams, curve: &Secp256k1<All>, privatekey: &PrivateKey) -> (TxOut, ScriptTypes) {
        let amount = params
            .value
            .unwrap_or_else(|| Amount::from_sat(rand::thread_rng().gen::<u64>()));

        let (script, script_type) = match params.script_pubkey {
            Some(script) => (script, ScriptTypes::P2PKH),
            None => ScriptBuf::random(
                params.script_params.unwrap_or_default(),
                curve,
                privatekey
            ),
        };

        let txout = TxOut {
            value: amount,
            script_pubkey: script
        };

        (txout, script_type)
    }
}