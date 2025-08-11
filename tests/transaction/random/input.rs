use bitcoin::{
    key::Secp256k1, NetworkKind, OutPoint, PrivateKey, PublicKey, ScriptBuf, Sequence, Transaction,
    TxIn, Witness,
};
use misfit_core::transaction::random::{
    input::{InputParams, RandomInput},
    script::{RandomScript, ScriptParams, ScriptTypes},
    transaction::{RandomTransacion, TxParams},
    witness::{RandomWitness, WitnessParams},
};
use secp256k1::rand::{self, Rng};

#[test]
fn test_random_input() {
    let input: TxIn = TxIn::random(InputParams::default());

    assert!(input.total_size() >= 1)
}

#[test]
fn test_random_input_with_specified_outpoint() {
    let input_tx = Transaction::random(TxParams::default());
    let vout = rand::thread_rng().gen_range(0..input_tx.output.len());

    let expected_outpoint: OutPoint = OutPoint {
        txid: input_tx.compute_txid(),
        vout: vout.try_into().unwrap(),
    };

    let input = TxIn::random(InputParams {
        outpoint: Some(expected_outpoint),
        ..Default::default()
    });

    assert!(input.previous_output.eq(&expected_outpoint))
}

#[test]
fn test_random_input_with_specified_script() {
    let (expected_script, expected_script_type) = ScriptBuf::random(ScriptParams {
        script_type: None,
        ..Default::default()
    });

    let input = TxIn::random(InputParams {
        script: Some((expected_script.clone(), expected_script_type)),
        ..Default::default()
    });

    assert!(input.script_sig.eq(&expected_script))
}

#[test]
fn test_random_input_with_specified_sequence() {
    let expected_sequence = Sequence(rand::thread_rng().gen::<u32>());

    let input = TxIn::random(InputParams {
        sequence: Some(expected_sequence),
        ..Default::default()
    });

    assert!(input.sequence.eq(&expected_sequence))
}

#[test]
fn test_random_input_with_specified_witness() {
    let expected_witness = Witness::random(WitnessParams {
        script: Some(ScriptBuf::random(ScriptParams {
            script_type: Some(ScriptTypes::P2WPKH),
            ..Default::default()
        })),
        ..Default::default()
    });

    let input = TxIn::random(InputParams {
        witness: Some(expected_witness.clone()),
        ..Default::default()
    });

    assert!(input.witness.eq(&expected_witness))
}

#[test]
fn test_random_input_with_specified_private_key() {
    let private_key: PrivateKey = PrivateKey::generate(NetworkKind::Main);
    let expected_public_key: PublicKey =
        PublicKey::from_private_key(&Secp256k1::new(), &private_key);

    let input = TxIn::random(InputParams {
        private_key: Some(private_key.clone()),
        script_params: Some(ScriptParams {
            script_type: Some(ScriptTypes::P2PK),
            private_key: Some(private_key.clone()),
        }),
        ..Default::default()
    });

    assert!(input
        .script_sig
        .p2pk_public_key()
        .unwrap()
        .eq(&expected_public_key))
}
