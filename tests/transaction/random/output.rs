use bitcoin::{key::Secp256k1, Amount, NetworkKind, PrivateKey, PublicKey, TxOut};
use misfit_core::transaction::random::{
    output::{OutputParams, RandomOutput},
    script::{ScriptParams, ScriptTypes},
};
use secp256k1::rand::{self, Rng};

#[test]
fn test_random_output() {
    let (output, _): (TxOut, ScriptTypes) = TxOut::random(OutputParams::default());

    assert!(output.value.to_sat() >= 1);
    assert!(!output.script_pubkey.is_empty());
}

#[test]
fn test_random_output_with_specified_value() {
    let expected_value: Amount = Amount::from_sat(rand::thread_rng().gen::<u64>());

    let (output, _): (TxOut, ScriptTypes) = TxOut::random(OutputParams {
        value: Some(expected_value),
        ..Default::default()
    });

    assert!(output.value.eq(&expected_value));
}

#[test]
fn test_random_output_with_specified_script_params() {
    let expected_script_type: ScriptTypes = match rand::thread_rng().gen_range(0..6) {
        0 => ScriptTypes::P2PK,
        1 => ScriptTypes::P2PKH,
        2 => ScriptTypes::P2SH,
        3 => ScriptTypes::P2TR,
        4 => ScriptTypes::P2TWEAKEDTR,
        5 => ScriptTypes::P2WPKH,
        _ => ScriptTypes::P2WSH,
    };

    let (_, script_type): (TxOut, ScriptTypes) = TxOut::random(OutputParams {
        script_params: Some(ScriptParams {
            script_type: Some(expected_script_type.clone()),
            ..Default::default()
        }),
        ..Default::default()
    });

    assert!(script_type.eq(&expected_script_type));
}

#[test]
fn test_random_output_with_specified_private_key() {
    let private_key: PrivateKey = PrivateKey::generate(NetworkKind::Main);
    let expected_public_key: PublicKey =
        PublicKey::from_private_key(&Secp256k1::new(), &private_key);

    let (output, script_type): (TxOut, ScriptTypes) = TxOut::random(OutputParams {
        script_params: Some(ScriptParams {
            script_type: Some(ScriptTypes::P2PK),
            private_key: Some(private_key),
        }),
        private_key: Some(private_key),
        ..Default::default()
    });

    assert!(script_type.eq(&ScriptTypes::P2PK));
    assert!(output
        .script_pubkey
        .p2pk_public_key()
        .unwrap()
        .eq(&expected_public_key));
}
