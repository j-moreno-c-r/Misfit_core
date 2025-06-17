use bitcoin::{Amount, ScriptBuf, TxOut, PrivateKey, secp256k1::{All, Secp256k1}};
use secp256k1::rand::{self, Rng};

use super::script::{RandomScript, ScriptParams, ScriptInfo, ScriptTypes};

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
    fn random(params: OutputParams, curve: &Secp256k1<All>, privatekey: &PrivateKey) -> TxOut;
}

impl RandomOutput for TxOut {
    fn random(params: OutputParams, curve: &Secp256k1<All>, privatekey: &PrivateKey) -> OutputInfo {
        let amount = params
            .value
            .unwrap_or_else(|| Amount::from_sat(rand::thread_rng().gen::<u64>()));

        let script_info = match params.script_pubkey {
            Some(script) => ScriptInfo {
                script,
                script_type: ScriptTypes::P2PKH, // Default type, você pode ajustar conforme necessário
            },
            None => ScriptBuf::random(
                params.script_params.unwrap_or_default(),
                curve,
                privatekey
            ),
        };

        OutputInfo {
            txout: TxOut {
                value: amount,
                script_pubkey: script_info.script,
            },
            script_type: script_info.script_type,
        }
    }
}