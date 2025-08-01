use std::io;
use std::io::Write;
use clap::{Parser, Subcommand};
use super::api::Generator;
use super::cli_functions::{
build_transaction_flags_vector,build_block_flags_and_config,
transaction_splitter, block_splitter, 
break_transaction, break_block, 
transaction, block,
help, clear, handle_result
};   
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
        #[arg(default_value = None)]
        txscount: Option<u32>,
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

pub struct TransactionFlags {
    pub version: bool,
    pub txid: bool,
    pub vout: bool,
    pub script_sig: bool,
    pub sequence: bool,
    pub amount: bool,
    pub script_pubkey: bool,
    pub witness: bool,
    pub locktime: bool,
    pub all: bool,
}

pub struct BlockFlagsConfig {
    pub version: bool,
    pub prev_hash: bool,
    pub merkle_root: bool,
    pub timestamp: bool,
    pub bits: bool,
    pub nonce: bool,
    pub all: bool,
    pub version_override: Option<i32>,
    pub timestamp_offset: Option<i64>,
    pub zero_hashes: bool,
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
            Commands::Tx { txscount, .. } => transaction(txscount), 
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
 