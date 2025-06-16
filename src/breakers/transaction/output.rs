use super::{script::corrupt_script, InvalidationFlag};
use bitcoin::{Amount, TxOut};
use std::collections::HashSet;

pub fn invalidate_output_in_place(
    output: &mut TxOut,
    flags: &HashSet<InvalidationFlag>,
    invalidate_all: bool,
) {
    // Invalidate amount
    if invalidate_all || flags.contains(&InvalidationFlag::OutputAmount) {
        let current_sats = output.value.to_sat();
        output.value = Amount::from_sat(u64::MAX - current_sats);
    }

    // Invalidate script_pubkey
    if invalidate_all || flags.contains(&InvalidationFlag::OutputScriptPubKey) {
        output.script_pubkey = corrupt_script(&output.script_pubkey);
    }
}
