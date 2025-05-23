use bitcoin::{
    hashes::Hash,
    key::{Keypair, Secp256k1, TweakedKeypair, TweakedPublicKey},
    NetworkKind, PrivateKey, PublicKey, ScriptBuf, ScriptHash, WScriptHash, XOnlyPublicKey,
};
use secp256k1::rand::{self, Rng};

pub enum ScriptTypes {
    P2PK,
    P2PKH,
    P2SH,
    P2TR,
    P2TWEAKEDTR,
    P2WPKH,
    P2WSH,
}

impl Clone for ScriptTypes {
    fn clone(&self) -> Self {
        match self {
            ScriptTypes::P2PK => ScriptTypes::P2PK,
            ScriptTypes::P2PKH => ScriptTypes::P2PKH,
            ScriptTypes::P2SH => ScriptTypes::P2SH,
            ScriptTypes::P2TR => ScriptTypes::P2TR,
            ScriptTypes::P2TWEAKEDTR => ScriptTypes::P2TWEAKEDTR,
            ScriptTypes::P2WPKH => ScriptTypes::P2WPKH,
            ScriptTypes::P2WSH => ScriptTypes::P2WSH,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

#[derive(Clone)]
pub struct ScriptParams {
    script_type: Option<ScriptTypes>,
}

impl Default for ScriptParams {
    fn default() -> Self {
        ScriptParams { script_type: None }
    }
}

pub trait RandomScript {
    fn random(params: ScriptParams) -> ScriptBuf;
}

impl RandomScript for ScriptBuf {
    fn random(params: ScriptParams) -> ScriptBuf {
        let script_type =
            params
                .script_type
                .unwrap_or_else(|| match rand::thread_rng().gen_range(0..6) {
                    0 => ScriptTypes::P2PK,
                    1 => ScriptTypes::P2PKH,
                    2 => ScriptTypes::P2SH,
                    3 => ScriptTypes::P2TR,
                    4 => ScriptTypes::P2TWEAKEDTR,
                    5 => ScriptTypes::P2WPKH,
                    _ => ScriptTypes::P2WSH,
                });

        match script_type {
            ScriptTypes::P2PK => ScriptBuf::new_p2pk(&PublicKey::from_private_key(
                &Secp256k1::new(),
                &PrivateKey::generate(NetworkKind::Main),
            )),
            ScriptTypes::P2PKH => ScriptBuf::new_p2pkh(
                &PublicKey::from_private_key(
                    &Secp256k1::new(),
                    &PrivateKey::generate(NetworkKind::Main),
                )
                .pubkey_hash(),
            ),
            ScriptTypes::P2SH => ScriptBuf::new_p2sh(&ScriptHash::all_zeros()),
            ScriptTypes::P2TR => ScriptBuf::new_p2tr(
                &Secp256k1::new(),
                XOnlyPublicKey::from_keypair(&Keypair::new(
                    &Secp256k1::new(),
                    &mut rand::thread_rng(),
                ))
                .0,
                None,
            ),
            ScriptTypes::P2TWEAKEDTR => ScriptBuf::new_p2tr_tweaked(
                TweakedPublicKey::from_keypair(TweakedKeypair::dangerous_assume_tweaked(
                    Keypair::new(&Secp256k1::new(), &mut rand::thread_rng()),
                )),
            ),
            ScriptTypes::P2WPKH => ScriptBuf::new_p2wpkh(
                &PublicKey::from_private_key(
                    &Secp256k1::new(),
                    &PrivateKey::generate(NetworkKind::Main),
                )
                .wpubkey_hash()
                .unwrap(),
            ),
            ScriptTypes::P2WSH => ScriptBuf::new_p2wsh(&WScriptHash::all_zeros()),
        }
    }
}
