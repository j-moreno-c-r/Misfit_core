use bitcoin::{hashes::Hash, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, Txid, Witness};
use secp256k1::rand::{self, Rng};

use super::{
    output::OutputParams,
    transaction::{RandomTransacion, TxParams},
};

#[derive(Default)]
pub struct InputParams {
    pub outpoint: Option<OutPoint>,
    pub script: Option<ScriptBuf>,
    pub sequence: Option<Sequence>,
    pub witness: Option<Witness>,
}


pub trait RandomInput {
    fn random(params: InputParams) -> TxIn;
}

impl RandomInput for TxIn {
    fn random(params: InputParams) -> TxIn {
        let outpoint = params.outpoint.unwrap_or_else(|| {
            // Create a random transaction for use as outpoint
            let tx_id = Transaction::random(TxParams {
                version: None,
                lock_time: None,
                input: Some(InputParams {
                    outpoint: Some(OutPoint {
                        txid: Txid::all_zeros(),
                        vout: rand::thread_rng().gen::<u32>(),
                    }),
                    script: None,
                    sequence: None,
                    witness: None,
                }),
                output: Some(OutputParams::default()),
            })
            .compute_txid();

            OutPoint {
                txid: tx_id,
                vout: rand::thread_rng().gen::<u32>(),
            }
        });
        let script = params.script.unwrap_or_default(); // TODO: When random, get script from outpoint
        let sequence = params
            .sequence
            .unwrap_or_else(|| Sequence(rand::thread_rng().gen::<u32>()));
        let witness = params.witness.unwrap_or_default();

        TxIn {
            previous_output: outpoint,
            script_sig: script,
            sequence,
            witness,
        }
    }
}
