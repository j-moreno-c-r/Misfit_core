use std::io;
use std::io::Write;
use clap::{Parser, Subcommand};
use crate::api::Generator;

#[derive(Parser)]
#[command(version, about, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Help,
    Clear,
    Exit,
    #[command(name = "decode-transaction")]
    DecodeTransaction {
        raw_transaction: String
    },
    #[command(name = "decode-block")]
    DecodeBlock {
        block_header: String
    },
    #[command(name = "break-transaction")]
    BreakTransaction {
        raw_transaction: String,
        #[arg(long, help = "Invalidate transaction version")]
        version: bool,
        #[arg(long, help = "Invalidate input transaction ID")]
        txid: bool,
        #[arg(long, help = "Invalidate input vout")]
        vout: bool,
        #[arg(long = "script-sig", help = "Invalidate input script signature")]
        script_sig: bool,
        #[arg(long, help = "Invalidate input sequence number")]
        sequence: bool,
        #[arg(long, help = "Invalidate output amount")]
        amount: bool,
        #[arg(long = "script-pubkey", help = "Invalidate output script pubkey")]
        script_pubkey: bool,
        #[arg(long, help = "Invalidate witness data")]
        witness: bool,
        #[arg(long, help = "Invalidate transaction locktime")]
        locktime: bool,
        #[arg(long, help = "Invalidate all transaction fields")]
        all: bool,
    },
    #[command(name = "break-block")]
    BreakBlock {
        block_header: String,
        #[arg(long, help = "Invalidate block version")]
        version: bool,
        #[arg(long = "prev-hash", help = "Invalidate previous block hash")]
        prev_hash: bool,
        #[arg(long = "merkle-root", help = "Invalidate merkle root")]
        merkle_root: bool,
        #[arg(long, help = "Invalidate timestamp")]
        timestamp: bool,
        #[arg(long, help = "Invalidate difficulty bits")]
        bits: bool,
        #[arg(long, help = "Invalidate nonce")]
        nonce: bool,
        #[arg(long, help = "Invalidate all block fields")]
        all: bool,
        #[arg(long, help = "Override version with specific value")]
        version_override: Option<i32>,
        #[arg(long, help = "Add/subtract seconds to timestamp")]
        timestamp_offset: Option<i64>,
        #[arg(long, help = "Use zero hashes instead of random")]
        zero_hashes: bool,
    },
    Tx {
        #[arg(default_value_t = 1)]
        txscount: u32,
        campuses: Vec<String>,
    },
    Block {
        #[arg(default_value_t = 1)]
        txscount: u32,
    },
    #[command(name = "regtest-start")]
    RegtestStart,
    #[command(name = "regtest-stop")]
    RegtestStop,
    #[command(name = "get-blockby-height")]
    GetBlockbyHeight {
        height: u64,
    },
}

struct TransactionFlags {
    version: bool,
    txid: bool,
    vout: bool,
    script_sig: bool,
    sequence: bool,
    amount: bool,
    script_pubkey: bool,
    witness: bool,
    locktime: bool,
    all: bool,
}

struct BlockFlagsConfig {
    version: bool,
    prev_hash: bool,
    merkle_root: bool,
    timestamp: bool,
    bits: bool,
    nonce: bool,
    all: bool,
    version_override: Option<i32>,
    timestamp_offset: Option<i64>,
    zero_hashes: bool,
}

pub fn handle() {
    let regtest_manager = Generator::regtest_invocation("bitcoinhos", "-regtest");

    loop {
        print!("-> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let args: Vec<&str> = input.split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        let cli = match Cli::try_parse_from(std::iter::once("").chain(args.iter().copied())) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };

        match cli.command {
            Commands::Help => help(),
            Commands::DecodeTransaction { raw_transaction } => transaction_splitter(raw_transaction),
            Commands::DecodeBlock { block_header } => block_splitter(block_header),
            Commands::BreakTransaction { 
                raw_transaction, 
                version, 
                txid, 
                vout, 
                script_sig, 
                sequence, 
                amount, 
                script_pubkey, 
                witness, 
                locktime, 
                all 
            } => {
                let flags = build_transaction_flags_vector(TransactionFlags {
                    version,
                    txid,
                    vout,
                    script_sig,
                    sequence,
                    amount,
                    script_pubkey,
                    witness,
                    locktime,
                    all,
                });
                break_transaction(raw_transaction, flags);
            },
            Commands::BreakBlock {
                block_header,
                version,
                prev_hash,
                merkle_root,
                timestamp,
                bits,
                nonce,
                all,
                version_override,
                timestamp_offset,
                zero_hashes,
            } => {
                let (flags, config) = build_block_flags_and_config(BlockFlagsConfig {
                    version,
                    prev_hash,
                    merkle_root,
                    timestamp,
                    bits,
                    nonce,
                    all,
                    version_override,
                    timestamp_offset,
                    zero_hashes,
                });
                break_block(block_header, flags, config);
            },
            Commands::Tx { txscount, .. } => transaction(txscount), // TODO: Implement params into transaction generator
            Commands::Block { txscount } => block(txscount),
            Commands::Clear => clear(),
            Commands::RegtestStart => handle_result(regtest_manager.start()),
            Commands::RegtestStop => handle_result(regtest_manager.stop()),
            Commands::GetBlockbyHeight { height } => {
                handle_result(regtest_manager.handle_getblockbyheight(height))
            }
            Commands::Exit => break
        }
    }
    println!("Program finalized 👋");
}
 
fn build_transaction_flags_vector(flags: TransactionFlags) -> Vec<String> {
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


fn build_block_flags_and_config(cfg: BlockFlagsConfig) -> (Vec<String>, Vec<String>) {
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

fn help() {
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

fn transaction_splitter(raw_transaction: String) {
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

fn block_splitter(block_header: String) {
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

fn break_transaction(raw_transaction: String, flags: Vec<String>) {
    if flags.is_empty() {
        println!("No invalidation flags specified. Use 'help' for usage information.");
        return;
    }
    
    let result = Generator::break_transaction(raw_transaction, flags);
    println!("🔨 Transaction Breaking Result:");
    println!("{result}");
}

fn break_block(block_header: String, flags: Vec<String>, config: Vec<String>) {
    if flags.is_empty() {
        println!("No invalidation flags specified. Use 'help' for usage information.");
        return;
    }
    
    let result = Generator::break_block(block_header, flags, config);
    println!("🔨 Block Breaking Result:");
    println!("{result}");
}

fn transaction(txscount: u32) {
    let transactions = Generator::transaction(txscount);
    println!("Transactions: {transactions}");
}

fn block(txscount: u32) {
    let block = Generator::block(Some(txscount), None);
    println!("Block: {block}");
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H"); 
    io::stdout().flush().unwrap();
}

fn handle_result(result: Result<(), Box<dyn std::error::Error>>) {
    if let Err(e) = result {
        eprintln!("Error: {e} 🚨");
    }
}