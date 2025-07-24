use std::collections::HashSet;
use bitcoin::{TxIn, Witness};
use super::{flags::InvalidationFlag, script::corrupt_script};

pub fn invalidate_input_in_place(
    input: &mut TxIn, 
    flags: &HashSet<InvalidationFlag>, 
    invalidate_all: bool
) {
    // Note: InputTxid invalidation is now handled at transaction level
    
    if invalidate_all || flags.contains(&InvalidationFlag::InputVout) {
        input.previous_output.vout ^= 1; // Flip last bit
    }
    
    // Invalidate script_sig
    if invalidate_all || flags.contains(&InvalidationFlag::InputScriptSig) {
        input.script_sig = corrupt_script(&input.script_sig);
    }
    
    // Invalidate sequence
    if invalidate_all || flags.contains(&InvalidationFlag::InputSequence) {
        input.sequence = bitcoin::Sequence(0xFFFFFFFF ^ input.sequence.0);
    }
    
    // Invalidate witness data
    if invalidate_all || flags.contains(&InvalidationFlag::WitnessData) {
        input.witness = corrupt_witness(&input.witness);
    }
}

pub fn corrupt_witness(witness: &Witness) -> Witness {
    let mut new_witness = witness.clone();
    if let Some(first_item) = new_witness.iter().next() {
        let mut corrupted = first_item.to_vec();
        if !corrupted.is_empty() {
            corrupted[0] = corrupted[0].wrapping_add(1);
        } else {
            corrupted.push(0x01);
        }
        let mut witness_stack = Vec::new();
        witness_stack.push(corrupted);
        // Add remaining items
        for item in witness.iter().skip(1) {
            witness_stack.push(item.to_vec());
        }
        new_witness = Witness::from_slice(&witness_stack);
    } else {
        // Empty witness, add a dummy item
        new_witness = Witness::from_slice(&[vec![0x01]]);
    }
    new_witness
}