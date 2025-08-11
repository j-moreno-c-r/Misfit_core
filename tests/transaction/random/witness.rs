use bitcoin::{
    ecdsa::Signature, key::Secp256k1, secp256k1::Message, sighash::SighashCache, EcdsaSighashType,
    NetworkKind, PrivateKey, PublicKey, ScriptBuf, Transaction, Witness,
};
use misfit_core::transaction::random::{
    script::{RandomScript, ScriptParams, ScriptTypes},
    transaction::{RandomTransacion, TxParams},
    witness::{RandomWitness, WitnessParams},
};
use secp256k1::rand::{self, Rng};

#[test]
fn test_random_witness() {
    let witness: Witness = Witness::random(WitnessParams::default());

    assert!(!witness.is_empty())
}

#[test]
fn test_random_witness_with_specified_params() {
    let private_key: PrivateKey = PrivateKey::generate(NetworkKind::Main);

    let pub_key: PublicKey = PublicKey::from_private_key(&Secp256k1::new(), &private_key);

    let transaction: Transaction = Transaction::random(TxParams::default());
    let vout = rand::thread_rng().gen_range(0..transaction.output.len());

    let (script, script_type) = ScriptBuf::random(ScriptParams {
        script_type: Some(ScriptTypes::P2WPKH),
        private_key: Some(private_key.clone()),
    });

    let sighash = SighashCache::new(&transaction)
        .p2wpkh_signature_hash(
            vout,
            &script,
            transaction.output[vout].value,
            EcdsaSighashType::All,
        )
        .unwrap();

    let sig: Signature = Signature {
        signature: Secp256k1::new().sign_ecdsa(
            &Message::from_digest_slice(&sighash[..]).unwrap(),
            &private_key.inner,
        ),
        sighash_type: EcdsaSighashType::All,
    };

    let expected_witness = Witness::p2wpkh(&sig, &pub_key.inner);

    let witness: Witness = Witness::random(WitnessParams {
        private_key: Some(private_key.clone()),
        transaction: Some(transaction.clone()),
        script: Some((script, script_type)),
        ..Default::default()
    });

    assert!(witness.eq(&expected_witness))
}
