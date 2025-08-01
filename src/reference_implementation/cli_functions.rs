use super::cli::{TransactionFlags, BlockFlagsConfig};
use super::api::Generator;
use std::io;
use std::io::Write;

pub fn build_transaction_flags_vector(flags: TransactionFlags) -> Vec<String> {
    let mut result = Vec::new();

    if flags.all {
        result.push("--all".to_string());
        return result;
    }
    if flags.version { result.push("--version".to_string()); }
    if flags.txid { result.push("--txid".to_string()); }
    if flags.vout { result.push("--vout".to_string()); }
    if flags.script_sig { result.push("--script-sig".to_string()); }
    if flags.sequence { result.push("--sequence".to_string()); }
    if flags.amount { result.push("--amount".to_string()); }
    if flags.script_pubkey { result.push("--script-pubkey".to_string()); }
    if flags.witness { result.push("--witness".to_string()); }
    if flags.locktime { result.push("--locktime".to_string()); }

    result
}

pub fn build_block_flags_and_config(cfg: BlockFlagsConfig) -> (Vec<String>, Vec<String>) {
    let mut flags = Vec::new();
    let mut config = Vec::new();

    if cfg.all {
        flags.push("--all".to_string());
    } else {
        if cfg.version { flags.push("--version".to_string()); }
        if cfg.prev_hash { flags.push("--prev-hash".to_string()); }
        if cfg.merkle_root { flags.push("--merkle-root".to_string()); }
        if cfg.timestamp { flags.push("--timestamp".to_string()); }
        if cfg.bits { flags.push("--bits".to_string()); }
        if cfg.nonce { flags.push("--nonce".to_string()); }
    }

    if let Some(override_val) = cfg.version_override {
        config.push(format!("--version-override={override_val}"));
    }
    if let Some(offset) = cfg.timestamp_offset {
        config.push(format!("--timestamp-offset={offset}"));
    }
    if cfg.zero_hashes {
        config.push("--zero-hashes".to_string());
    }

    (flags, config)
}

pub fn transaction_splitter(raw_transaction: String) {
    match Generator::decode_raw_transaction(raw_transaction) {
        Ok(decoded) => {
            println!("Version: {}", decoded.version);
            println!("Locktime: {}", decoded.lock_time);
            println!("Input count: {:#?}", decoded.input);
            println!("Output count: {:#?}", decoded.output);
        },
        Err(e) => {
            eprintln!("Error decoding transaction: {e}");
        }
    }
}

pub fn block_splitter(block_header: String) {
    match Generator::decoder_block_header(block_header) {
        Ok(header) => {
            println!("Version: {}", header.version.to_consensus());
            println!("Previous Block: {}", header.prev_blockhash);
            println!("Merkle Root: {}", header.merkle_root);
            println!("Timestamp: {}", header.time);
            println!("Bits: 0x{:08x}", header.bits.to_consensus());
            println!("Nonce: {}", header.nonce);
            println!("Block Hash: {}", header.block_hash());
        },
        Err(e) => {
            eprintln!("Error decoding block header: {e}");
        }
    }
}

pub fn break_transaction(raw_transaction: String, flags: Vec<String>) {
    if flags.is_empty() {
        println!("No invalidation flags specified. Use 'help' for usage information.");
        return;
    }
    
    let result = Generator::break_transaction(raw_transaction, flags);
    println!("🔨 Transaction Breaking Result:");
    println!("{result}");
}

pub fn break_block(block_header: String, flags: Vec<String>, config: Vec<String>) {
    if flags.is_empty() {
        println!("No invalidation flags specified. Use 'help' for usage information.");
        return;
    }
    
    let result = Generator::break_block(block_header, flags, config);
    println!("🔨 Block Breaking Result:");
    println!("{result}");
}

pub fn transaction(txscount: u32) {
    let transactions = Generator::transaction(txscount);
    println!("Transactions: {transactions}");
}

pub fn help() {
    println!("Available commands:\n");
    println!("\x1b[32m[Utills]\x1b[0m");
    println!("help                                  - Show help message");
    println!("clear                                 - Clear terminal screen");
    println!("exit");
    println!("\x1b[32m[Decode]\x1b[0m");
    println!("decode-transaction <raw_tx>           - Decode a raw transaction");
    println!("decode-block <block_header>           - Decode a block header");
    println!(" ");
    println!("\x1b[32m[Break/Invalidate]\x1b[0m");
    println!("  \x1b[34mbreak-transaction <raw_tx> [FLAGS]\x1b[0m   - Break/invalidate specific fields of a transaction");
    println!("  Available flags:");
    println!("    --version         - Invalidate transaction version");
    println!("    --txid            - Invalidate input transaction ID");
    println!("    --vout            - Invalidate input vout");
    println!("    --script-sig      - Invalidate input script signature");
    println!("    --sequence        - Invalidate input sequence number");
    println!("    --amount          - Invalidate output amount");
    println!("    --script-pubkey   - Invalidate output script pubkey");
    println!("    --witness         - Invalidate witness data");
    println!("    --locktime        - Invalidate transaction locktime");
    println!("    --all             - Invalidate all transaction fields");
    println!("\x1b[34mbreak-block <block_header> [FLAGS]\x1b[0m - Break/invalidate specific fields of a block");
    println!("  Available flags:");
    println!("    --version         - Invalidate block version");
    println!("    --prev-hash       - Invalidate previous block hash");
    println!("    --merkle-root     - Invalidate merkle root");
    println!("    --timestamp       - Invalidate timestamp");
    println!("    --bits            - Invalidate difficulty bits");
    println!("    --nonce           - Invalidate nonce");
    println!("    --all             - Invalidate all block fields");
    println!("  Configuration options:");
    println!("    --version-override <value>  - Override version with specific value");
    println!("    --timestamp-offset <secs>   - Add/subtract seconds to timestamp");
    println!("    --zero-hashes               - Use zero hashes instead of random");
    println!("\x1b[32m[Generate]\x1b[0m");
    println!("tx <txscount> [params...]             - Generate one or more transactions");
    println!(
        "block <txscount>                      - Generate new block with one or more transactions"
    );
    println!("\x1b[32m[Regtest]\x1b[0m");
    println!(
        "get-blockby-height <height>           - Get a block at specific height in the regtest"
    );
    println!("regtest-start                         - Start the regtest node");
    println!("regtest-stop                          - Stop the regtest node (please remember to stop before closing the program)");
}

pub fn block(txscount: Option<u32>) {
    let block = Generator::block(txscount, None);
    println!("Block: {block}");
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H"); 
    io::stdout().flush().unwrap();
}

pub fn handle_result(result: Result<(), Box<dyn std::error::Error>>) {
    if let Err(e) = result {
        eprintln!("Error: {e} 🚨");
    }
}