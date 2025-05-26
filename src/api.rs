use bitcoin::consensus::encode;
use bitcoin::block::Header;
use bitcoin::Transaction;
use std::collections::HashSet;
use misfit_core::transaction::random::transaction::TxParams;
use misfit_core::transaction::generator::GenerateTx;
use misfit_core::block::generate_blocks::GenerateBlock;
use misfit_core::breakers::{decoder_tools, transaction_breaker, block_breaker};

pub struct Generator {}

impl Generator {
    pub fn block(tx_count: u32) -> String {
        let mut raw_tx: Vec<String> = vec![];
        let mut txid: Vec<String> = vec![];

        for _c in 0..tx_count {
            let tx = GenerateTx::valid_random(TxParams::default());
            let raw_transaction = hex::encode(encode::serialize(&tx)).to_string();
            let tx_id = tx.compute_txid().to_string();

            raw_tx.push(raw_transaction);
            txid.push(tx_id);
        }

        let block_header = GenerateBlock::new(txid.clone());

        [
            format!("Blockheader Info 🧊: {:#?} ", block_header),
            format!("Raw transactions used in it:{:#?}", raw_tx),
            format!("Used Txids: {:#?}", txid),
        ]
        .join("\n---\n")
    }

    // TODO: Implement params into transaction generator
    pub fn transaction(count: u32) -> String {
        let mut raw_tx: Vec<String> = vec![];
        let mut txid: Vec<String> = vec![];

        for _c in 0..count {
            let tx = GenerateTx::valid_random(TxParams::default());
            let raw_transaction = hex::encode(encode::serialize(&tx)).to_string();
            let tx_id = tx.compute_txid().to_string();

            raw_tx.push(raw_transaction);
            txid.push(tx_id);
        }

        [
            format!("Raw Transactions: {:#?}", raw_tx),
            format!("TXIDs: {:#?}", txid),
        ]
        .join("\n---\n")
    }

    pub fn decode_raw_transaction(raw_tx: String) -> Result<Transaction, Box<dyn std::error::Error>> {
        let decoder = decoder_tools::BitcoinTransactionDecoder::new();
        let decoded = decoder.decode_hex(&raw_tx);
        decoded    
    }

    pub fn decoder_block_header(block_header: String) -> Result<Header, Box<dyn std::error::Error>> {
        decoder_tools::BlockUtils::decode_header_from_hex(&block_header)
    }

    pub fn break_transaction(transaction: String, cli_flags: Vec<String>) -> String {
        // Convert CLI flags to InvalidationFlag HashSet
        let invalidation_flags = Self::parse_cli_flags_to_invalidation_flags(cli_flags);
        
        if invalidation_flags.is_empty() {
            return "No invalidation flags specified. Use 'help' for usage information.".to_string();
        }

        // Decode the transaction
        let decoded_tx = match Self::decode_raw_transaction(transaction.clone()) {
            Ok(tx) => tx,
            Err(e) => return format!("Error decoding transaction: {}", e),
        };

        // Create invalid version based on specified flags
        let invalid_tx = transaction_breaker::TransactionInvalidator::invalidate(decoded_tx, &invalidation_flags);

        // Build the result string
        let mut result = String::new();
        
        // List which fields are being invalidated
        result.push_str("Invalidating the following fields:\n");
        
        if invalidation_flags.contains(&transaction_breaker::InvalidationFlag::All) {
            result.push_str("  - ALL FIELDS\n");
        } else {
            for flag in &invalidation_flags {
                match flag {
                    transaction_breaker::InvalidationFlag::Version => result.push_str("  - Transaction Version\n"),
                    transaction_breaker::InvalidationFlag::InputTxid => result.push_str("  - Input TXIDs\n"),
                    transaction_breaker::InvalidationFlag::InputVout => result.push_str("  - Input Vouts\n"),
                    transaction_breaker::InvalidationFlag::InputScriptSig => result.push_str("  - Input Script Signatures\n"),
                    transaction_breaker::InvalidationFlag::InputSequence => result.push_str("  - Input Sequences\n"),
                    transaction_breaker::InvalidationFlag::OutputAmount => result.push_str("  - Output Amounts\n"),
                    transaction_breaker::InvalidationFlag::OutputScriptPubKey => result.push_str("  - Output Script PubKeys\n"),
                    transaction_breaker::InvalidationFlag::WitnessData => result.push_str("  - Witness Data\n"),
                    transaction_breaker::InvalidationFlag::Locktime => result.push_str("  - Locktime\n"),
                    _ => {}
                }
            }
        }

        // Display results
        result.push_str(&format!("\nInputed Transaction:\n{}\n\n", transaction));
        result.push_str(&format!("Invalidated Transaction:\n{:#?}", invalid_tx));
        
        result
    }

    pub fn break_block(block_header: String, cli_flags: Vec<String>, cli_config: Vec<String>) -> String {
        // Parse CLI flags to BlockField vector
        let block_fields = Self::parse_cli_flags_to_block_fields(cli_flags);
        
        if block_fields.is_empty() {
            return "No invalidation flags specified. Use 'help' for usage information.".to_string();
        }

        // Parse configuration options
        let processing_config = Self::parse_cli_config_to_processing_config(cli_config, block_fields);

        // Decode the block header
        let decoded_header = match Self::decoder_block_header(block_header.clone()) {
            Ok(header) => header,
            Err(e) => return format!("Error decoding block header: {}", e),
        };

        // Create block from header for processing
        let original_block = decoder_tools::BlockUtils::create_minimal_block_from_header(decoded_header.clone());

        // Process the block using BlockProcessor
        let processor = block_breaker::BlockProcessor::new(processing_config.clone());
        let broken_block = processor.process_block(&original_block);

        // Build the result string
        let mut result = String::new();
        
        // List which fields are being invalidated
        result.push_str("Breaking the following block fields:\n");
        
        if processing_config.fields_to_modify.contains(&block_breaker::BlockField::All) {
            result.push_str("  - ALL FIELDS\n");
        } else {
            for field in &processing_config.fields_to_modify {
                match field {
                    block_breaker::BlockField::Version => result.push_str("  - Block Version\n"),
                    block_breaker::BlockField::PrevBlockHash => result.push_str("  - Previous Block Hash\n"),
                    block_breaker::BlockField::MerkleRoot => result.push_str("  - Merkle Root\n"),
                    block_breaker::BlockField::Timestamp => result.push_str("  - Timestamp\n"),
                    block_breaker::BlockField::Bits => result.push_str("  - Difficulty Bits\n"),
                    block_breaker::BlockField::Nonce => result.push_str("  - Nonce\n"),
                    _ => {}
                }
            }
        }

        // Add configuration info
        if let Some(version_override) = processing_config.version_override {
            result.push_str(&format!("  - Version Override: {}\n", version_override));
        }
        if let Some(timestamp_offset) = processing_config.timestamp_offset {
            result.push_str(&format!("  - Timestamp Offset: {} seconds\n", timestamp_offset));
        }
        if !processing_config.randomize_hashes {
            result.push_str("  - Using zero hashes instead of random\n");
        }

        // Display original header info
        result.push_str(&format!("\nOriginal Block Header:\n"));
        result.push_str(&format!("  Version: {}\n", decoded_header.version.to_consensus()));
        result.push_str(&format!("  Previous Block: {}\n", decoded_header.prev_blockhash));
        result.push_str(&format!("  Merkle Root: {}\n", decoded_header.merkle_root));
        result.push_str(&format!("  Timestamp: {}\n", decoded_header.time));
        result.push_str(&format!("  Bits: 0x{:08x}\n", decoded_header.bits.to_consensus()));
        result.push_str(&format!("  Nonce: {}\n", decoded_header.nonce));
        result.push_str(&format!("  Block Hash: {}\n", decoded_header.block_hash()));

        // Display broken header info
        result.push_str(&format!("\nBroken Block Header:\n"));
        result.push_str(&format!("  Version: {}\n", broken_block.header.version.to_consensus()));
        result.push_str(&format!("  Previous Block: {}\n", broken_block.header.prev_blockhash));
        result.push_str(&format!("  Merkle Root: {}\n", broken_block.header.merkle_root));
        result.push_str(&format!("  Timestamp: {}\n", broken_block.header.time));
        result.push_str(&format!("  Bits: 0x{:08x}\n", broken_block.header.bits.to_consensus()));
        result.push_str(&format!("  Nonce: {}\n", broken_block.header.nonce));
        result.push_str(&format!("  Block Hash: {}\n", broken_block.header.block_hash()));

        // Display hex representation of broken header
        let broken_header_hex = hex::encode(encode::serialize(&broken_block.header));
        result.push_str(&format!("\nBroken Block Header (Hex):\n{}\n", broken_header_hex));
        
        result
    }

    /// Convert CLI flags (from clap) to InvalidationFlag HashSet
    fn parse_cli_flags_to_invalidation_flags(cli_flags: Vec<String>) -> HashSet<transaction_breaker::InvalidationFlag> {
        let mut flags = HashSet::new();

        for flag in cli_flags {
            let invalidation_flag = match flag.as_str() {
                "--version" => Some(transaction_breaker::InvalidationFlag::Version),
                "--txid" => Some(transaction_breaker::InvalidationFlag::InputTxid),
                "--vout" => Some(transaction_breaker::InvalidationFlag::InputVout),
                "--script-sig" => Some(transaction_breaker::InvalidationFlag::InputScriptSig),
                "--sequence" => Some(transaction_breaker::InvalidationFlag::InputSequence),
                "--amount" => Some(transaction_breaker::InvalidationFlag::OutputAmount),
                "--script-pubkey" => Some(transaction_breaker::InvalidationFlag::OutputScriptPubKey),
                "--witness" => Some(transaction_breaker::InvalidationFlag::WitnessData),
                "--locktime" => Some(transaction_breaker::InvalidationFlag::Locktime),
                "--all" => Some(transaction_breaker::InvalidationFlag::All),
                _ => {
                    println!("Warning: Unknown flag '{}' ignored", flag);
                    None
                }
            };

            if let Some(flag) = invalidation_flag {
                flags.insert(flag);
            }
        }

        flags
    }

    /// Convert CLI flags to BlockField vector
    fn parse_cli_flags_to_block_fields(cli_flags: Vec<String>) -> Vec<block_breaker::BlockField> {
        let mut fields = Vec::new();

        for flag in cli_flags {
            let block_field = match flag.as_str() {
                "--version" => Some(block_breaker::BlockField::Version),
                "--prev-hash" => Some(block_breaker::BlockField::PrevBlockHash),
                "--merkle-root" => Some(block_breaker::BlockField::MerkleRoot),
                "--timestamp" => Some(block_breaker::BlockField::Timestamp),
                "--bits" => Some(block_breaker::BlockField::Bits),
                "--nonce" => Some(block_breaker::BlockField::Nonce),
                "--all" => Some(block_breaker::BlockField::All),
                _ => {
                    println!("Warning: Unknown block field flag '{}' ignored", flag);
                    None
                }
            };

            if let Some(field) = block_field {
                fields.push(field);
            }
        }

        fields
    }

    /// Convert CLI config options to ProcessingConfig
    fn parse_cli_config_to_processing_config(
        cli_config: Vec<String>, 
        fields: Vec<block_breaker::BlockField>
    ) -> block_breaker::ProcessingConfig {
        let mut config = block_breaker::ProcessingConfig {
            fields_to_modify: fields,
            version_override: None,
            timestamp_offset: None,
            randomize_hashes: true, // default to random hashes
        };

        for config_option in cli_config {
            if config_option.starts_with("--version-override=") {
                if let Some(value_str) = config_option.strip_prefix("--version-override=") {
                    if let Ok(value) = value_str.parse::<i32>() {
                        config.version_override = Some(value);
                    } else {
                        println!("Warning: Invalid version override value '{}' ignored", value_str);
                    }
                }
            } else if config_option.starts_with("--timestamp-offset=") {
                if let Some(value_str) = config_option.strip_prefix("--timestamp-offset=") {
                    if let Ok(value) = value_str.parse::<i64>() {
                        config.timestamp_offset = Some(value);
                    } else {
                        println!("Warning: Invalid timestamp offset value '{}' ignored", value_str);
                    }
                }
            } else if config_option == "--zero-hashes" {
                config.randomize_hashes = false;
            } else {
                println!("Warning: Unknown config option '{}' ignored", config_option);
            }
        }

        config
    }

/* 
    pub fn break_block(flags: Vec<String>) -> String {
        // TODO: Implement block breaking functionality
        "Block breaking not yet implemented".to_string()
    }
*/
}