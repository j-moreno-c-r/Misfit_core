use bitcoin::{
    hashes::Hash, NetworkKind, OutPoint, PrivateKey, ScriptBuf, Sequence, Transaction, TxIn, Txid,
    Witness,
};
use secp256k1::rand::{self, Rng};

use crate::transaction::random::{witness::{RandomWitness, WitnessParams, RandomTaprootWitness, TaprootWitnessParams},};

use super::{
    script::{RandomScript, ScriptParams, ScriptTypes},
    transaction::{RandomTransacion, TxParams},
};


#[derive(Default)] 
pub struct InputParams {
    pub outpoint: Option<OutPoint>,
    pub script: Option<(ScriptBuf, ScriptTypes)>,
    pub sequence: Option<Sequence>,
    pub witness: Option<Witness>,
    pub script_params: Option<ScriptParams>,
    pub private_key: Option<PrivateKey>,
}

 
pub trait RandomInput {
    fn random(params: InputParams) -> TxIn;
}

impl RandomInput for TxIn {
    fn random(params: InputParams) -> TxIn {
        let mut witness_params = WitnessParams::default();

        let private_key = params
            .private_key
            .unwrap_or_else(|| PrivateKey::generate(NetworkKind::Main));
        witness_params.private_key = Some(private_key);

        let (script_buf, script_type) = params.script.unwrap_or_else(|| {
            ScriptBuf::random(params.script_params.unwrap_or(ScriptParams {
                script_type: None,
                private_key: Some(private_key),
            }))
        });
        witness_params.script = Some((script_buf.clone(), script_type.clone()));

        let outpoint = params.outpoint.unwrap_or_else(|| {
            let mut random_tx_params = TxParams::default();
            let random_input_params = InputParams {
                witness: Some(Witness::default()),
                outpoint: Some(OutPoint {
                    txid: Txid::all_zeros(),
                    vout: rand::thread_rng().gen::<u32>(),
                }),
                ..Default::default()
            };

            random_tx_params.input = Some(random_input_params);

            let random_input_tx = Transaction::random(random_tx_params);

            witness_params.transaction = Some(random_input_tx.clone());

            let vout = rand::thread_rng().gen_range(0..random_input_tx.output.len());
            witness_params.vout = Some(vout);

            OutPoint {
                txid: random_input_tx.compute_txid(),
                vout: vout.try_into().unwrap(),
            }
        });

        let witness = params.witness.unwrap_or_else(|| {
            match script_type {
                ScriptTypes::P2WPKH | ScriptTypes::P2SH => {
                    <Witness as RandomWitness>::random(witness_params)
                }
                ScriptTypes::P2TR | ScriptTypes::P2TWEAKEDTR => {
                    let taproot_params = TaprootWitnessParams::default();
                    <Witness as RandomTaprootWitness>::random(taproot_params)                }
                _ => {
                    Witness::default()
                }
            }
        });

        let sequence = params
            .sequence
            .unwrap_or_else(|| Sequence(rand::thread_rng().gen::<u32>()));

        TxIn {
            previous_output: outpoint,
            script_sig: script_buf,
            sequence,
            witness,
        }
    }
}