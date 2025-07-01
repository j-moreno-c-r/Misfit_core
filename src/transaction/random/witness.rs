use bitcoin::{
    ecdsa::Signature,
    hashes::Hash,
    secp256k1::{Message, Secp256k1},
    sighash::{EcdsaSighashType, SighashCache},
    NetworkKind, OutPoint, PrivateKey, PublicKey, ScriptBuf, SegwitV0Sighash, Transaction, Txid,
    Witness,
    key::{Keypair, TapTweak},//TweakedKeypair
    Amount,
    sighash::{Prevouts, TapSighashType},
    TxOut, 
};
use secp256k1::rand::{self, Rng};

use crate::transaction::random::{
    input::InputParams,
    script::{RandomScript, ScriptParams, ScriptTypes},
    transaction::{RandomTransacion, TxParams},
};



#[derive(Default, Debug, Clone)]
pub struct TaprootWitnessParams {
    pub transaction: Option<Transaction>,
    pub vout: Option<usize>,
    pub utxo: Option<TxOut>,
    pub keypair: Option<Keypair>,
    pub script_type: Option<ScriptTypes>,
}
pub trait RandomTaprootWitness {
    fn random(params: TaprootWitnessParams) -> Witness;
}

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
            let mut random_input_params = InputParams::default();

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
            let mut script_params = ScriptParams::default();

            script_params.script_type = Some(if rand::thread_rng().gen_bool(0.5) {
                ScriptTypes::P2WSH
            } else {
                ScriptTypes::P2WPKH
            });

            ScriptBuf::random(script_params)
        });

        let private_key = params
            .private_key
            .unwrap_or_else(|| PrivateKey::generate(NetworkKind::Main));

        let pub_key = PublicKey::from_private_key(&Secp256k1::new(), &private_key);

        let sighash = match script_type {
            ScriptTypes::P2WPKH => SighashCache::new(&transaction)
                .p2wpkh_signature_hash(vout, &script, amount, EcdsaSighashType::All)
                .unwrap(),

            ScriptTypes::P2WSH => SighashCache::new(&transaction)
                .p2wsh_signature_hash(vout, &script, amount, EcdsaSighashType::All)
                .unwrap(),

            _ => SegwitV0Sighash::all_zeros(),
        };

        let sig = Signature {
            signature: Secp256k1::new().sign_ecdsa(
                &Message::from_digest_slice(&sighash[..]).unwrap(),
                &private_key.inner,
            ),
            sighash_type: EcdsaSighashType::All,
        };

        match script_type {
            ScriptTypes::P2WPKH => Witness::p2wpkh(&sig, &pub_key.inner),
            ScriptTypes::P2WSH => {
                let mut sig_ser = sig.serialize().to_vec();
                sig_ser.push(EcdsaSighashType::All as u8);

                let mut witness = Witness::new();
                witness.push(sig_ser);
                witness.push(script.as_bytes());

                witness
            }
            _ => Witness::default(),
        }
    }
}


impl RandomTaprootWitness for Witness {
    fn random(mut _params: TaprootWitnessParams) -> Witness {
        let secp = Secp256k1::new();

        let keypair = _params.keypair.unwrap_or_else(|| {
            let sk = bitcoin::secp256k1::SecretKey::new(&mut rand::thread_rng());
            Keypair::from_secret_key(&secp, &sk)
        });

        let tx = _params.transaction.unwrap_or_else(|| {
            Transaction {
                version: bitcoin::transaction::Version::TWO,
                lock_time: bitcoin::absolute::LockTime::ZERO,
                input: vec![bitcoin::TxIn::default()],
                output: vec![],
            }
        });

        let vout = _params.vout.unwrap_or(0);
        let utxo = _params.utxo.unwrap_or_else(|| TxOut {
            value: Amount::from_sat(100_000),
            script_pubkey: bitcoin::ScriptBuf::default(),
        });
        let binding = [utxo.clone()];
        let prevouts = Prevouts::All(&binding);
        let mut cache = SighashCache::new(&tx);
        let sighash = cache
            .taproot_key_spend_signature_hash(
                vout,
                &prevouts,
                TapSighashType::Default,
            )
            .expect("sighash");

        let tweaked = keypair.tap_tweak(&secp, None);
        let msg = Message::from_digest(sighash.to_byte_array());
        let schnorr_sig = secp.sign_schnorr(&msg, &tweaked.to_keypair());

        let signature = bitcoin::taproot::Signature {
            signature: schnorr_sig,
            sighash_type: TapSighashType::Default,
        };

        Witness::p2tr_key_spend(&signature)
    }
}