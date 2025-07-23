use bitcoin::block::Header;
use bitcoin::consensus::encode;
use bitcoin::Transaction;
use misfit_core::block::breakers::block as BlockBreaker;
use misfit_core::transaction::breakers as TxBreaker;
use misfit_core::block::generator::GenerateBlock;
use misfit_core::block::random::block::BlockParams;
use misfit_core::decoder_tools;
use misfit_core::regtest_pack::regtest::RegtestManager;
use misfit_core::transaction::generator::GenerateTx;
use misfit_core::transaction::random::input::InputParams;
use misfit_core::transaction::random::script::{ScriptParams, ScriptTypes};
use misfit_core::transaction::random::transaction::TxParams;
use std::collections::HashSet;

pub struct Generator {}

impl Generator {
    pub fn block(tx_count: u32) -> String {
        let mut txs: Vec<Transaction> = vec![];
        let mut raw_tx: Vec<String> = vec![];
        let mut tx_ids: Vec<String> = vec![];

        for _c in 0..tx_count {
        let mut tx_params = TxParams::default();
        let tx_input_params = InputParams {
            script_params: Some(ScriptParams {
                script_type: Some(ScriptTypes::P2WPKH),
                private_key: None,
            }),
            ..Default::default()
        };

        tx_params.input = Some(tx_input_params);

        let tx_info = GenerateTx::valid_random(tx_params);
        let raw_transaction = hex::encode(encode::serialize(&tx_info)).to_string();
        let txid = tx_info.compute_txid().to_string();

        txs.push(tx_info);
        raw_tx.push(raw_transaction);
        tx_ids.push(txid);
}

        let (block, height) = GenerateBlock::valid_random(BlockParams {
            header: None,
            txs: Some(txs),
            height: None,
        });

        [
            format!("{:#?} ", block.header),
            format!("Block Height: {height}"),
            format!("Block Header encoded: {:#?}", encode::serialize_hex(&block.header)),
            format!("Raw txs: {raw_tx:#?}"),
            format!("TxID: {tx_ids:#?}"),
        ]
        .join("\n---\n")
    }

    pub fn transaction(count: u32) -> String {
        let mut raw_tx: Vec<String> = vec![];
        let mut txid: Vec<String> = vec![];

        for _c in 0..count {
        let mut tx_params = TxParams::default();
        let tx_input_params = InputParams {
            script_params: Some(ScriptParams {
                script_type: Some(ScriptTypes::P2WPKH),
                private_key: None,
            }),
            ..Default::default()
        };

        tx_params.input = Some(tx_input_params);

        let tx_info = GenerateTx::valid_random(tx_params);
        let raw_transaction = hex::encode(encode::serialize(&tx_info)).to_string();
        let tx_id = tx_info.compute_txid().to_string();

        raw_tx.push(raw_transaction);
        txid.push(tx_id);
    }

        [
            format!("Raw Transactions: {raw_tx:#?}"),
            format!("TXIDs: {txid:#?}"),
        ]
        .join("\n---\n")
    }

    pub fn decode_raw_transaction(
        raw_tx: String,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let decoder = decoder_tools::BitcoinTransactionDecoder::new();
        decoder.decode_hex(&raw_tx)
    }

    pub fn decoder_block_header(
        block_header: String,
    ) -> Result<Header, Box<dyn std::error::Error>> {
        decoder_tools::BlockUtils::decode_header_from_hex(&block_header)
    }
    pub fn regtest_invocation(name_of_wallet: &str, mode_of_cli: &str) -> RegtestManager {
        RegtestManager::new(name_of_wallet, mode_of_cli)
    }

    pub fn break_transaction(transaction: String, cli_flags: Vec<String>) -> String {
        // Convert CLI flags to InvalidationFlag HashSet
        let invalidation_flags = Self::parse_cli_flags_to_invalidation_flags(cli_flags);

        if invalidation_flags.is_empty() {
            return "No invalidation flags specified. Use 'help' for usage information."
                .to_string();
        }

        // Decode the transaction
        let decoded_tx = match Self::decode_raw_transaction(transaction.clone()) {
            Ok(tx) => tx,
            Err(e) => return format!("Error decoding transaction: {e}"),
        };

        // Create invalid version based on specified flags
        let invalid_tx = TxBreaker::transaction::TransactionInvalidator::invalidate(
            decoded_tx,
            &invalidation_flags,
        );

        // Build the result string
        let mut result = String::new();

        // List which fields are being invalidated
        result.push_str("Invalidating the following fields:\n");

        if invalidation_flags.contains(&TxBreaker::flags::InvalidationFlag::All) {
            result.push_str("  - ALL FIELDS\n");
        } else {
            for flag in &invalidation_flags {
                match flag {
                    TxBreaker::flags::InvalidationFlag::Version => {
                        result.push_str("  - Transaction Version\n")
                    }
                    TxBreaker::flags::InvalidationFlag::InputTxid => {
                        result.push_str("  - Input TXIDs\n")
                    }
                    TxBreaker::flags::InvalidationFlag::InputVout => {
                        result.push_str("  - Input Vouts\n")
                    }
                    TxBreaker::flags::InvalidationFlag::InputScriptSig => {
                        result.push_str("  - Input Script Signatures\n")
                    }
                    TxBreaker::flags::InvalidationFlag::InputSequence => {
                        result.push_str("  - Input Sequences\n")
                    }
                    TxBreaker::flags::InvalidationFlag::OutputAmount => {
                        result.push_str("  - Output Amounts\n")
                    }
                    TxBreaker::flags::InvalidationFlag::OutputScriptPubKey => {
                        result.push_str("  - Output Script PubKeys\n")
                    }
                    TxBreaker::flags::InvalidationFlag::WitnessData => {
                        result.push_str("  - Witness Data\n")
                    }
                    TxBreaker::flags::InvalidationFlag::Locktime => {
                        result.push_str("  - Locktime\n")
                    }
                    _ => {}
                }
            }
        }

        // Display results
        result.push_str(&format!("\nInputed Transaction:\n{transaction}\n\n"));
        result.push_str(&format!("Invalidated Transaction:\n{invalid_tx:#?}"));
        result.push_str(&format!(
            "Invalidated Raw Transaction:\n{:#?}\n\n",
            encode::serialize_hex(&invalid_tx)
        ));

        result
    }

    pub fn break_block(
        block_header: String,
        cli_flags: Vec<String>,
        cli_config: Vec<String>,
    ) -> String {
        // Parse CLI flags to BlockField vector
        let block_fields = Self::parse_cli_flags_to_block_fields(cli_flags);

        if block_fields.is_empty() {
            return "No invalidation flags specified. Use 'help' for usage information."
                .to_string();
        }

        // Parse configuration options
        let processing_config =
            Self::parse_cli_config_to_processing_config(cli_config, block_fields);

        // Decode the block header
        let decoded_header = match Self::decoder_block_header(block_header.clone()) {
            Ok(header) => header,
            Err(e) => return format!("Error decoding block header: {e}"),
        };

        // Create block from header for processing
        let original_block =
            decoder_tools::BlockUtils::create_minimal_block_from_header(decoded_header);

        // Process the block using BlockProcessor
        let processor = BlockBreaker::BlockProcessor::new(processing_config.clone());
        let broken_block = processor.process_block(&original_block);

        // Build the result string
        let mut result = String::new();

        // List which fields are being invalidated
        result.push_str("Breaking the following block fields:\n");

        if processing_config
            .fields_to_modify
            .contains(&BlockBreaker::BlockField::All)
        {
            result.push_str("  - ALL FIELDS\n");
        } else {
            for field in &processing_config.fields_to_modify {
                match field {
                    BlockBreaker::BlockField::Version => result.push_str("  - Block Version\n"),
                    BlockBreaker::BlockField::PrevBlockHash => {
                        result.push_str("  - Previous Block Hash\n")
                    }
                    BlockBreaker::BlockField::MerkleRoot => result.push_str("  - Merkle Root\n"),
                    BlockBreaker::BlockField::Timestamp => result.push_str("  - Timestamp\n"),
                    BlockBreaker::BlockField::Bits => result.push_str("  - Difficulty Bits\n"),
                    BlockBreaker::BlockField::Nonce => result.push_str("  - Nonce\n"),
                    _ => {}
                }
            }
        }

        // Add configuration info
        if let Some(version_override) = processing_config.version_override {
            result.push_str(&format!("  - Version Override: {version_override}\n"));
        }
        if let Some(timestamp_offset) = processing_config.timestamp_offset {
            result.push_str(&format!(
                "  - Timestamp Offset: {timestamp_offset} seconds\n"
            ));
        }
        if !processing_config.randomize_hashes {
            result.push_str("  - Using zero hashes instead of random\n");
        }

        // Display original header info
        result.push_str("\nOriginal Block Header:\n");
        result.push_str(&format!(
            "  Version: {}\n",
            decoded_header.version.to_consensus()
        ));
        result.push_str(&format!(
            "  Previous Block: {}\n",
            decoded_header.prev_blockhash
        ));
        result.push_str(&format!("  Merkle Root: {}\n", decoded_header.merkle_root));
        result.push_str(&format!("  Timestamp: {}\n", decoded_header.time));
        result.push_str(&format!(
            "  Bits: 0x{:08x}\n",
            decoded_header.bits.to_consensus()
        ));
        result.push_str(&format!("  Nonce: {}\n", decoded_header.nonce));
        result.push_str(&format!("  Block Hash: {}\n", decoded_header.block_hash()));

        // Display broken header info
        result.push_str("\nBroken Block Header:\n");
        result.push_str(&format!(
            "  Version: {}\n",
            broken_block.header.version.to_consensus()
        ));
        result.push_str(&format!(
            "  Previous Block: {}\n",
            broken_block.header.prev_blockhash
        ));
        result.push_str(&format!(
            "  Merkle Root: {}\n",
            broken_block.header.merkle_root
        ));
        result.push_str(&format!("  Timestamp: {}\n", broken_block.header.time));
        result.push_str(&format!(
            "  Bits: 0x{:08x}\n",
            broken_block.header.bits.to_consensus()
        ));
        result.push_str(&format!("  Nonce: {}\n", broken_block.header.nonce));
        result.push_str(&format!(
            "  Block Hash: {}\n",
            broken_block.header.block_hash()
        ));

        // Display hex representation of broken header
        let broken_header_hex = hex::encode(encode::serialize(&broken_block.header));
        result.push_str(&format!(
            "\nBroken Block Header (Hex):\n{broken_header_hex}\n"
        ));

        result
    }

    pub fn parse_cli_flags_to_invalidation_flags(
        cli_flags: Vec<String>,
    ) -> HashSet<TxBreaker::flags::InvalidationFlag> {
        let mut flags = HashSet::new();

        for flag in cli_flags {
            let invalidation_flag = match flag.as_str() {
                "--version" => Some(TxBreaker::flags::InvalidationFlag::Version),
                "--txid" => Some(TxBreaker::flags::InvalidationFlag::InputTxid),
                "--vout" => Some(TxBreaker::flags::InvalidationFlag::InputVout),
                "--script-sig" => Some(TxBreaker::flags::InvalidationFlag::InputScriptSig),
                "--sequence" => Some(TxBreaker::flags::InvalidationFlag::InputSequence),
                "--amount" => Some(TxBreaker::flags::InvalidationFlag::OutputAmount),
                "--script-pubkey" => Some(TxBreaker::flags::InvalidationFlag::OutputScriptPubKey),
                "--witness" => Some(TxBreaker::flags::InvalidationFlag::WitnessData),
                "--locktime" => Some(TxBreaker::flags::InvalidationFlag::Locktime),
                "--all" => Some(TxBreaker::flags::InvalidationFlag::All),
                _ => {
                    println!("Warning: Unknown flag '{flag}' ignored");
                    None
                }
            };

            if let Some(flag) = invalidation_flag {
                flags.insert(flag);
            }
        }

        flags
    }

    pub fn parse_cli_flags_to_block_fields(
        cli_flags: Vec<String>,
    ) -> Vec<BlockBreaker::BlockField> {
        let mut fields = Vec::new();

        for flag in cli_flags {
            let block_field = match flag.as_str() {
                "--version" => Some(BlockBreaker::BlockField::Version),
                "--prev-hash" => Some(BlockBreaker::BlockField::PrevBlockHash),
                "--merkle-root" => Some(BlockBreaker::BlockField::MerkleRoot),
                "--timestamp" => Some(BlockBreaker::BlockField::Timestamp),
                "--bits" => Some(BlockBreaker::BlockField::Bits),
                "--nonce" => Some(BlockBreaker::BlockField::Nonce),
                "--all" => Some(BlockBreaker::BlockField::All),
                _ => {
                    println!("Warning: Unknown block field flag '{flag}' ignored");
                    None
                }
            };

            if let Some(field) = block_field {
                fields.push(field);
            }
        }

        fields
    }

    pub fn parse_cli_config_to_processing_config(
        cli_config: Vec<String>,
        fields: Vec<BlockBreaker::BlockField>,
    ) -> BlockBreaker::ProcessingConfig {
        let mut config = BlockBreaker::ProcessingConfig {
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
                        println!(
                            "Warning: Invalid version override value '{value_str}' ignored"
                        );
                    }
                }
            } else if config_option.starts_with("--timestamp-offset=") {
                if let Some(value_str) = config_option.strip_prefix("--timestamp-offset=") {
                    if let Ok(value) = value_str.parse::<i64>() {
                        config.timestamp_offset = Some(value);
                    } else {
                        println!(
                            "Warning: Invalid timestamp offset value '{value_str}' ignored"
                        );
                    }
                }
            } else if config_option == "--zero-hashes" {
                config.randomize_hashes = false;
            } else {
                println!("Warning: Unknown config option '{config_option}' ignored");
            }
        }

        config
    }
}
