use super::{input::*, locktime::*, output::*, version::*, InvalidationFlag};
use bitcoin::{consensus::deserialize, Transaction};
use std::collections::HashSet;

// Transaction processor with Bitcoin Core Transaction struct
pub struct TransactionInvalidator;

impl TransactionInvalidator {
    pub fn invalidate(mut tx: Transaction, flags: &HashSet<InvalidationFlag>) -> Transaction {
        let should_invalidate_all = flags.contains(&InvalidationFlag::All);

        // Invalidate transaction structure (affects txid) - do this first
        if should_invalidate_all || flags.contains(&InvalidationFlag::InputTxid) {
            Self::corrupt_txid(&mut tx);
        }

        // Invalidate version
        if should_invalidate_all || flags.contains(&InvalidationFlag::Version) {
            tx.version = invalidate_version(tx.version);
        }

        // Invalidate locktime
        if should_invalidate_all || flags.contains(&InvalidationFlag::Locktime) {
            tx.lock_time = invalidate_locktime(tx.lock_time);
        }

        // Invalidate inputs
        for input in tx.input.iter_mut() {
            invalidate_input_in_place(input, flags, should_invalidate_all);
        }

        // Invalidate outputs
        for output in tx.output.iter_mut() {
            invalidate_output_in_place(output, flags, should_invalidate_all);
        }

        tx
    }

    // Helper methods
    fn corrupt_txid(tx: &mut Transaction) -> bitcoin::Txid {
        // Remove the last input if there are multiple inputs
        if tx.input.len() > 1 {
            tx.input.pop();
        } else if !tx.input.is_empty() {
            // If only one input, corrupt its previous output
            tx.input[0].previous_output.vout = tx.input[0].previous_output.vout.wrapping_add(1);
        }

        // Compute and return the new transaction ID
        tx.compute_txid()
    }
}

pub struct BitcoinTransactionDecoder;

impl BitcoinTransactionDecoder {
    pub fn new() -> Self {
        Self
    }

    /// Decode a hex string directly to bitcoin::Transaction
    pub fn decode_hex(&self, hex_string: &str) -> Result<Transaction, Box<dyn std::error::Error>> {
        let clean_hex = hex_string.trim().replace(" ", "").to_lowercase();
        let bytes = hex::decode(&clean_hex)?;
        self.decode_bytes(&bytes)
    }

    /// Decode bytes directly to bitcoin::Transaction
    pub fn decode_bytes(&self, bytes: &[u8]) -> Result<Transaction, Box<dyn std::error::Error>> {
        let tx: Transaction = deserialize(bytes)?;
        Ok(tx)
    }

    /// Helper method to check if transaction has witness data
    pub fn has_witness_data(&self, tx: &Transaction) -> bool {
        tx.input.iter().any(|input| !input.witness.is_empty())
    }

    /// Helper method to get SegWit marker and flag
    pub fn get_segwit_flags(&self, tx: &Transaction) -> (u8, u8) {
        if self.has_witness_data(tx) {
            (0x00, 0x01) // SegWit marker and flag
        } else {
            (0x00, 0x00) // No SegWit
        }
    }
}

impl Default for BitcoinTransactionDecoder {
    fn default() -> Self {
        Self::new()
    }
}

// Define available invalidation flags - moved to flags.rs

pub fn parse_flags(args: Vec<String>) -> HashSet<InvalidationFlag> {
    let mut flags = HashSet::new();

    // Display help and exit if requested
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage();
        std::process::exit(0);
    }

    // No flags provided - show help
    if args.len() <= 1 {
        print_usage();
        std::process::exit(1);
    }

    // Parse all provided flags
    for arg in args.iter().skip(1) {
        if !arg.starts_with("--") {
            continue;
        }

        let flag_str = &arg[2..]; // Remove "--" prefix
        if let Some(flag) = InvalidationFlag::from_str(flag_str) {
            flags.insert(flag);
        } else {
            println!("Warning: Unknown flag '{}' ignored", arg);
        }
    }

    flags
}

fn print_usage() {
    println!("Bitcoin Transaction Invalidator");
    println!("Usage: btc-invalidator [FLAGS]");
    println!("\nAvailable flags:");
    println!("  --all           Invalidate all transaction fields");
    println!("  --version       Invalidate transaction version");
    println!("  --txid          Invalidate input transaction ID");
    println!("  --vout          Invalidate input vout");
    println!("  --script-sig    Invalidate input script signature");
    println!("  --sequence      Invalidate input sequence number");
    println!("  --amount        Invalidate output amount");
    println!("  --script-pubkey Invalidate output script pubkey");
    println!("  --witness       Invalidate witness data");
    println!("  --locktime      Invalidate transaction locktime");
    println!("  --help          Print this help message");
    println!("\nExample: btc-invalidator --txid --amount --locktime");
}
