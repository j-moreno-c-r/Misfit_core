use bitcoin::{Amount, ScriptBuf, TxOut};
use secp256k1::rand::{self, Rng};

use super::script::{RandomScript, ScriptParams};

#[derive(Default)]
pub struct OutputParams {
    pub value: Option<Amount>,
    pub(crate) script_pubkey: Option<ScriptBuf>,
    pub script_params: Option<ScriptParams>,
}

pub trait RandomOutput {
    fn random(params: OutputParams) -> TxOut;
}

impl RandomOutput for TxOut {
    fn random(params: OutputParams) -> TxOut {
        // TODO: Fee estimator
        // TODO: Amount random value needs to be more than the sum of inputs and fee
        let amount = params
            .value
            .unwrap_or_else(|| Amount::from_sat(rand::thread_rng().gen::<u64>()));
        let script = params
            .script_pubkey
            .unwrap_or_else(|| ScriptBuf::random(params.script_params.unwrap_or_default()));

        TxOut {
            value: amount,
            script_pubkey: script,
        }
    }
}
