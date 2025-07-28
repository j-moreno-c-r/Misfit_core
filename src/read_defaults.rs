use serde_json::{self, Value};
use std::fs;
use bitcoin::{transaction::{Version},absolute::{LockTime,Height},Amount};
use misfit_core::transaction::random::{input::InputParams,script::{ScriptParams, ScriptTypes},transaction::TxParams};
use misfit_core::block::random::block::BlockParams;


pub fn match_transaction_defaults() -> TxParams {
    let defaults = read_defaults();

    let tx_defaults = &defaults["transaction"];

    let version = none_if_zero(&tx_defaults["version"]).map(Version);

    let lock_time = none_if_zero(&tx_defaults["lock_time"]).map(|v| LockTime::Blocks(Height::from_consensus(v as u32).unwrap()));

    let block_height = none_if_zero(&tx_defaults["block_height"]).map(|v| v as u32);

    let private_key = None;

    let input_defaults = &tx_defaults["input"];
    let input = Some(InputParams {
        outpoint: None, 
        script: None,
        sequence: None,
        witness: None,
        script_params: Some(ScriptParams {
            script_type: match none_if_empty_string(&input_defaults["script_params"]["script_type"]).as_deref() {
                Some("p2pk") => Some(ScriptTypes::P2PK),
                Some("p2pkh") => Some(ScriptTypes::P2PKH),
                Some("p2sh") => Some(ScriptTypes::P2SH),
                Some("p2wpkh") => Some(ScriptTypes::P2WPKH),
                Some("p2wpsh") => Some(ScriptTypes::P2WSH),
                Some("p2tr") => Some(ScriptTypes::P2TR),
                Some("p2tweakedt") => Some(ScriptTypes::P2TWEAKEDTR),
                _ => None,
            },
            private_key: None,
        }),
        private_key: None,
    });

    let output_defaults = &tx_defaults["output"];
    let value_i32 = none_if_zero(&output_defaults["value"]).unwrap_or(100000);
    let value = Some(Amount::from_sat(value_i32 as u64));
    let output = Some(misfit_core::transaction::random::output::OutputParams {
        value,
        script_params: Some(ScriptParams {
            script_type: match none_if_empty_string(&output_defaults["script_params"]["script_type"]).as_deref() {
                Some("p2pkh") => Some(ScriptTypes::P2PKH),
                Some("p2wpkh") => Some(ScriptTypes::P2WPKH),
                Some("p2sh") => Some(ScriptTypes::P2SH),
                Some("p2tr") => Some(ScriptTypes::P2TR),
                _ => None,
            },
            private_key: None,
        }),
        private_key: None,
    });

    TxParams {
        version,
        lock_time,
        input,
        output,
        private_key,
        block_height,
    }
}


pub fn match_block_defaults(txs:Option<i32>) -> (BlockParams,i32) {
    let defaults = read_defaults();

    let block_defaults = &defaults["block"];

    let header = None;  

    let txs_count:i32 = (txs).unwrap_or(none_if_zero(&block_defaults["transactions"]["count"]).unwrap());

    let height = none_if_zero(&block_defaults["height"]).map(|v| v as u32);

    (BlockParams {
        header,
        txs: None,
        height,
    }, txs_count)
}
pub fn read_defaults() -> Value {
    let data = fs::read_to_string("src/defaults.json").expect("Not Possible to read defaults.json");
    let value: Value = serde_json::from_str(&data).expect("Error to deserialize defaults.json");
    value
}

pub fn none_if_zero(v: &Value) -> Option<i32> {
    match v {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i == 0 {
                    None
                } else {
                    Some(i as i32)
                }
            } else {
                None
            }
        }
        Value::String(s) => {
            if let Ok(i) = s.parse::<i32>() {
                if i == 0 {
                    None
                } else {
                    Some(i)
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn none_if_empty_string(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => {
            if s.is_empty() || s == "0" {
                None
            } else {
                Some(s.clone())
            }
        }
        _ => None,
    }
}