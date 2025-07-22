use bitcoin::{
    ecdsa::Signature,
    hashes::Hash,
    key::Keypair,
    secp256k1::{Message, Secp256k1},
    sighash::{EcdsaSighashType, SighashCache},
    sighash::{Prevouts, TapSighashType},
    NetworkKind,
    OutPoint,
    PrivateKey,
    PublicKey,
    ScriptBuf,
    Transaction,
    Txid,
    Witness,
};
use secp256k1::rand::{self, Rng};

use crate::transaction::random::{
    input::InputParams,
    script::{RandomScript, ScriptParams, ScriptTypes},
    transaction::{RandomTransacion, TxParams},
};

#[derive(Default, Debug, Clone)]
pub struct WitnessParams {
    pub transaction: Option<Transaction>,
    pub vout: Option<usize>,
    pub script: Option<(ScriptBuf, ScriptTypes)>,
    pub private_key: Option<PrivateKey>,
}

pub trait RandomWitness {
    fn random(params: WitnessParams) -> Witness;
}

impl RandomWitness for Witness {
    fn random(params: WitnessParams) -> Witness {
        let transaction = params.transaction.unwrap_or_else(|| {
            let mut random_tx_params = TxParams::default();
            let mut random_input_params = InputParams {
                witness: Some(Witness::default()),
                outpoint: Some(OutPoint {
                    txid: Txid::all_zeros(),
                    vout: rand::thread_rng().gen::<u32>(),
                }),
                ..Default::default()
            };

            random_input_params.witness = Some(Witness::default());
            random_input_params.outpoint = Some(OutPoint {
                txid: Txid::all_zeros(),
                vout: rand::thread_rng().gen::<u32>(),
            });

            random_tx_params.input = Some(random_input_params);

            Transaction::random(random_tx_params)
        });

        let vout = params
            .vout
            .unwrap_or_else(|| rand::thread_rng().gen_range(0..transaction.output.len()));

        let amount = transaction.output[vout].value;

        let (script, script_type) = params.script.unwrap_or_else(|| {
            let mut script_params = ScriptParams {
            script_type: Some(match rand::thread_rng().gen_range(0..3) {
                0 => ScriptTypes::P2TR,
                1 => ScriptTypes::P2TWEAKEDTR,
                2 => ScriptTypes::P2WPKH,
                _ => ScriptTypes::P2WSH,
            }),
            ..Default::default()
};

            script_params.script_type = Some(match rand::thread_rng().gen_range(0..3) {
                0 => ScriptTypes::P2TR,
                1 => ScriptTypes::P2TWEAKEDTR,
                2 => ScriptTypes::P2WPKH,
                _ => ScriptTypes::P2WSH,
            });

            ScriptBuf::random(script_params)
        });

        let private_key = params
            .private_key
            .unwrap_or_else(|| PrivateKey::generate(NetworkKind::Main));

        let pub_key = PublicKey::from_private_key(&Secp256k1::new(), &private_key);

        match script_type {
            ScriptTypes::P2WPKH => {
                let sighash = SighashCache::new(&transaction)
                    .p2wpkh_signature_hash(vout, &script, amount, EcdsaSighashType::All)
                    .unwrap();

                let sig = Signature {
                    signature: Secp256k1::new().sign_ecdsa(
                        &Message::from_digest_slice(&sighash[..]).unwrap(),
                        &private_key.inner,
                    ),
                    sighash_type: EcdsaSighashType::All,
                };

                Witness::p2wpkh(&sig, &pub_key.inner)
            }

            ScriptTypes::P2WSH => {
                let sighash = SighashCache::new(&transaction)
                    .p2wsh_signature_hash(vout, &script, amount, EcdsaSighashType::All)
                    .unwrap();

                let sig = Signature {
                    signature: Secp256k1::new().sign_ecdsa(
                        &Message::from_digest_slice(&sighash[..]).unwrap(),
                        &private_key.inner,
                    ),
                    sighash_type: EcdsaSighashType::All,
                };

                let mut sig_ser = sig.serialize().to_vec();
                sig_ser.push(EcdsaSighashType::All as u8);

                let mut witness = Witness::new();
                witness.push(sig_ser);
                witness.push(script.as_bytes());

                witness
            }

            ScriptTypes::P2TR | ScriptTypes::P2TWEAKEDTR => {
                let sighash = SighashCache::new(&transaction)
                    .taproot_key_spend_signature_hash(
                        vout,
                        &Prevouts::All(&[transaction.tx_out(vout).unwrap()]),
                        TapSighashType::Default,
                    )
                    .unwrap();

                let sig = bitcoin::taproot::Signature {
                    signature: Secp256k1::new().sign_schnorr(
                        &Message::from_digest_slice(&sighash[..]).unwrap(),
                        &Keypair::from_secret_key(&Secp256k1::new(), &private_key.inner),
                    ),
                    sighash_type: TapSighashType::Default,
                };

                Witness::p2tr_key_spend(&sig)
            }

            _ => Witness::default(),
        }
    }
}
