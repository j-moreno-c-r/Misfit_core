use bitcoin::{Amount, NetworkKind, PrivateKey, ScriptBuf, TxOut};
use secp256k1::rand::{self, Rng};

use super::script::{RandomScript, ScriptParams, ScriptTypes};

pub struct OutputParams {
    pub value: Option<Amount>,
    pub script_params: Option<ScriptParams>,
    pub private_key: Option<PrivateKey>,
}

pub struct OutputInfo {
    pub txout: TxOut,
    pub script_type: ScriptTypes,
}

impl Default for OutputParams {
    fn default() -> Self {
        OutputParams {
            value: None,
            script_params: None,
            private_key: None,
        }
    }
}

pub trait RandomOutput {
    fn random(params: OutputParams) -> (TxOut, ScriptTypes);
}

impl RandomOutput for TxOut {
    fn random(params: OutputParams) -> (TxOut, ScriptTypes) {
        let amount = params
            .value
            .unwrap_or_else(|| Amount::from_sat(rand::thread_rng().gen::<u64>()));

        let private_key = params
            .private_key
            .unwrap_or_else(|| PrivateKey::generate(NetworkKind::Main));

        let script_params = params.script_params.unwrap_or(ScriptParams {
            script_type: None,
            private_key: Some(private_key),
        });

        let (script, script_type) = ScriptBuf::random(script_params);

        let txout = TxOut {
            value: amount,
            script_pubkey: script,
        };

        (txout, script_type)
    }
}
