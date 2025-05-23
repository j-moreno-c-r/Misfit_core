use std::io;
use std::io::Write;

use clap::{Parser, Subcommand};

use crate::generator::{generator::Generator, regtest::RegtestManager};

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
    Transaction {
        #[arg(default_value_t = 1)]
        txscount: u32,
        campuses: Vec<String>,
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
    RegtestStart,
    RegtestStop,
    GetBlockbyHeight {
        height: u64,
    },
}

pub fn handle() {
    let regtest_manager = RegtestManager::new("bitcoinhos", "-regtest");

    loop {
        print!("> ");
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
                eprintln!("Error: {}", e);
                continue;
            }
        };

        match cli.command {
            Commands::Help => help(),
            Commands::Transaction { txscount, .. } => transaction(txscount), // TODO: Implement params into transaction generator
            Commands::Tx { txscount, .. } => transaction(txscount), // TODO: Implement params into transaction generator
            Commands::Block { txscount } => block(txscount), // TODO: Implement args
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

fn help() {
    println!("Available commands:\n");
    println!("[Utils]");
    println!("help                                  - Show help message");
    println!("clear                                 - Clear terminal screen");
    println!("exit");
    println!("");
    println!("[Generate]");
    println!("transaction");
    println!("tx <txscount> [params...]             - Generate one or more transactions");
    println!(
        "block <txscount>                      - Generate new block with one or more transactions"
    );
    println!("");
    println!("[Regtest]");
    println!(
        "get-blockby-height <height>           - Get a block at specific height in the regtest"
    );
    println!("regtest-start                         - Start the regtest node");
    println!("regtest-stop                          - Stop the regtest node(please rember stop before close the program)");
}

// TODO: Implement params into transaction generator
fn transaction(txscount: u32) {
    let transactions = Generator::transaction(txscount);
    println!("Transactions: {}", transactions);
}

fn block(txscount: u32) {
    // TODO: Implement args
    let block = Generator::block(txscount);
    println!("Block: {}", block);
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn handle_result(result: Result<(), Box<dyn std::error::Error>>) {
    if let Err(e) = result {
        eprintln!("Error: {} 🚨", e);
    }
}
