use bitcoin::ScriptBuf;
use misfit_core::transaction::random::script::{RandomScript, ScriptParams, ScriptTypes};
use secp256k1::rand::{self, Rng};

#[test]
fn test_random_script() {
    let (script, _) = ScriptBuf::random(ScriptParams::default());

    assert!(!script.is_empty());
}

#[test]
fn test_random_script_with_specified_type() {
    let expected_script_type: ScriptTypes = match rand::thread_rng().gen_range(0..6) {
        0 => ScriptTypes::P2PK,
        1 => ScriptTypes::P2PKH,
        2 => ScriptTypes::P2SH,
        3 => ScriptTypes::P2TR,
        4 => ScriptTypes::P2TWEAKEDTR,
        5 => ScriptTypes::P2WPKH,
        _ => ScriptTypes::P2WSH,
    };

    let (_, script_type) = ScriptBuf::random(ScriptParams {
        script_type: Some(expected_script_type.clone()),
        ..Default::default()
    });

    assert!(script_type.eq(&expected_script_type));
}
