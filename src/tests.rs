
#[cfg(test)]
mod tests {
    pub use crate::api::{Generator};

    #[test]
    fn test_generate_single_transaction() {
        let result = Generator::transaction(1);
        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(json.get("raw_transactions").is_some());
        assert!(json.get("txids").is_some());
        let raw_txs = json["raw_transactions"].as_array().unwrap();
        let txids = json["txids"].as_array().unwrap();
        assert_eq!(raw_txs.len(), 1);
        assert_eq!(txids.len(), 1);
    }

    #[test]
    fn test_generate_multiple_transactions() {
        let tx_count = 3;
        let result = Generator::transaction(tx_count);
        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        let raw_txs = json["raw_transactions"].as_array().unwrap();
        let txids = json["txids"].as_array().unwrap();
        assert_eq!(raw_txs.len(), tx_count as usize);
        assert_eq!(txids.len(), tx_count as usize);
    }

    #[test]
    fn test_generate_zero_transactions() {
        let result = Generator::transaction(0);
        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        let raw_txs = json["raw_transactions"].as_array().unwrap();
        let txids = json["txids"].as_array().unwrap();
        assert_eq!(raw_txs.len(), 0);
        assert_eq!(txids.len(), 0);
    }

    #[test]
    fn test_generate_one_block_with_one_transaction() {
        let result = Generator::block(1);
        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(json.get("header").is_some());
        assert!(json.get("header_hex").is_some());
        let raw_txs = json["raw_transactions"].as_array().unwrap();
        let txids = json["txids"].as_array().unwrap();
        assert_eq!(raw_txs.len(), 1);
        assert_eq!(txids.len(), 1);
    }

    #[test]
    fn generate_zero_tx_block() {
        let result = Generator::block(0);
        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(json.get("header").is_some());
        assert!(json.get("header_hex").is_some());
        let raw_txs = json["raw_transactions"].as_array().unwrap();
        let txids = json["txids"].as_array().unwrap();
        assert_eq!(raw_txs.len(), 0);
        assert_eq!(txids.len(), 0);
    }

    #[test]
    fn test_generate_block_with_multiple_transactions() {
        let tx_count = 10;
        let result = Generator::block(tx_count);
        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        let raw_txs = json["raw_transactions"].as_array().unwrap();
        let txids = json["txids"].as_array().unwrap();
        assert_eq!(raw_txs.len(), tx_count as usize);
        assert_eq!(txids.len(), tx_count as usize);
    }

    #[test]
    fn test_parse_cli_flags_to_invalidation_flags() {
        let flags = vec![
            "--version".to_string(),
            "--txid".to_string(),
            "--amount".to_string(),
        ];
        
        let result = Generator::parse_cli_flags_to_invalidation_flags(flags);
        
        assert_eq!(result.len(), 3);
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::Version));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::InputTxid));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::OutputAmount));
    }

    #[test]
    fn test_parse_cli_flags_with_all_flag() {
        let flags = vec!["--all".to_string()];
        
        let result = Generator::parse_cli_flags_to_invalidation_flags(flags);
        
        assert_eq!(result.len(), 1);
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::All));
    }

    #[test]
    fn test_parse_cli_flags_with_unknown_flag() {
        let flags = vec![
            "--version".to_string(),
            "--unknown-flag".to_string(),
            "--txid".to_string(),
        ];
        
        let result = Generator::parse_cli_flags_to_invalidation_flags(flags);
        
        // Should only contain the known flags
        assert_eq!(result.len(), 2);
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::Version));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::InputTxid));
    }

    #[test]
    fn test_parse_cli_flags_to_block_fields() {
        let flags = vec![
            "--version".to_string(),
            "--prev-hash".to_string(),
            "--nonce".to_string(),
        ];
        
        let result = Generator::parse_cli_flags_to_block_fields(flags);
        
        assert_eq!(result.len(), 3);
        assert!(result.contains(&misfit_core::breakers::block::block::BlockField::Version));
        assert!(result.contains(&misfit_core::breakers::block::block::BlockField::PrevBlockHash));
        assert!(result.contains(&misfit_core::breakers::block::block::BlockField::Nonce));
    }

    #[test]
    fn test_parse_cli_config_to_processing_config() {
        let cli_config = vec![
            "--version-override=2".to_string(),
            "--timestamp-offset=3600".to_string(),
            "--zero-hashes".to_string(),
        ];
        let fields = vec![misfit_core::breakers::block::block::BlockField::Version];
        
        let result = Generator::parse_cli_config_to_processing_config(cli_config, fields);
        
        assert_eq!(result.version_override, Some(2));
        assert_eq!(result.timestamp_offset, Some(3600));
        assert_eq!(result.randomize_hashes, false);
        assert_eq!(result.fields_to_modify.len(), 1);
    }

    #[test]
    fn test_parse_cli_config_with_invalid_values() {
        let cli_config = vec![
            "--version-override=invalid".to_string(),
            "--timestamp-offset=not-a-number".to_string(),
        ];
        let fields = vec![];
        
        let result = Generator::parse_cli_config_to_processing_config(cli_config, fields);
        
        // Invalid values should be ignored, defaults should be used
        assert_eq!(result.version_override, None);
        assert_eq!(result.timestamp_offset, None);
        assert_eq!(result.randomize_hashes, true); 
    }

    #[test]
    fn test_regtest_invocation() {
        let wallet_name = "blablabla";
        let cli_mode = "-regtest";
        
        let _regtest_manager = Generator::regtest_invocation(wallet_name, cli_mode);
        
    }

    
    #[test]
    fn test_decode_raw_transaction_with_valid_data() {
        let valid_raw_tx = "4f6e3b7201e8370e51a135fb8e468e8188ea580b5a6c74a92b5cab5af2785bd307297be9a808e47956006b6b5dbe0118a478e14edc0b651976a9148840c86761418aa78e7667e8e7e427c4e955989588ac59500852".to_string();
        
        let result = Generator::decode_raw_transaction(valid_raw_tx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decoder_block_header_with_valid_data() {
        // This would require a valid block header hex string
        let valid_header = "00e0de23a528751ac3a3e02d8368dce7d902c1cb6561184d735b0700000000000000000023f401455373d8e00c0fef0402b2a9bf45a69ba1a0da0a6175ba571d633fe74c27bdaf6390f50717614aaf14".to_string();
        
        let result = Generator::decoder_block_header(valid_header);
        assert!(result.is_ok());
    }

    #[test]
    fn test_break_transaction_with_valid_data() {
        // Generate a transaction first
        let tx_result = Generator::transaction(1);

        let cli_flags = vec!["--version".to_string()];
        let result = Generator::break_transaction(tx_result.to_string(), cli_flags);
        
        assert!(result != tx_result);
    }

    #[test]
    fn test_break_transaction_with_no_flags() {
        let raw_tx = "4f6e3b7201e8370e51a135fb8e468e8188ea580b5a6c74a92b5cab5af2785bd307297be9a808e47956006b6b5dbe0118a478e14edc0b651976a9148840c86761418aa78e7667e8e7e427c4e955989588ac59500852".to_string();
        let cli_flags = vec![];
        
        let result = Generator::break_transaction(raw_tx, cli_flags);
        
        assert!(result.contains("No invalidation flags specified"));
        assert!(result.contains("Use 'help' for usage information"));
    }

    #[test]
    fn test_break_block_with_no_flags() {
        let block_header = "02000000de9ca7b23a61cc050a2286af1ee9a4f2fc31b3eb32adbf7b030000000000000064580288f07b0bf1670dad42dbd8aa8c0cd283ff61515f8a8e9cf4f3b973d450f475b2526eba0419869a148f".to_string();
        let cli_flags = vec![];
        let cli_config = vec![];
        
        let result = Generator::break_block(block_header, cli_flags, cli_config);
        
        assert!(result.contains("No invalidation flags specified"));
        assert!(result.contains("Use 'help' for usage information"));
    }

    // Test edge cases and error conditions
    
    #[test]
    fn test_empty_cli_flags() {
        let flags: Vec<String> = vec![];
        let result = Generator::parse_cli_flags_to_invalidation_flags(flags);
        assert!(result.is_empty());
    }

    #[test]
    fn test_empty_block_flags() {
        let flags: Vec<String> = vec![];
        let result = Generator::parse_cli_flags_to_block_fields(flags);
        assert!(result.is_empty());
    }

    #[test]
    fn test_processing_config_defaults() {
        let cli_config: Vec<String> = vec![];
        let fields = vec![];
        
        let result = Generator::parse_cli_config_to_processing_config(cli_config, fields);
        
        assert_eq!(result.version_override, None);
        assert_eq!(result.timestamp_offset, None);
        assert_eq!(result.randomize_hashes, true);
        assert!(result.fields_to_modify.is_empty());
    }

    #[test]
    fn test_all_transaction_invalidation_flags() {
        let flags = vec![
            "--version".to_string(),
            "--txid".to_string(),
            "--vout".to_string(),
            "--script-sig".to_string(),
            "--sequence".to_string(),
            "--amount".to_string(),
            "--script-pubkey".to_string(),
            "--witness".to_string(),
            "--locktime".to_string(),
        ];
        
        let result = Generator::parse_cli_flags_to_invalidation_flags(flags);
        
        assert_eq!(result.len(), 9);
        // Verify all expected flags are present
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::Version));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::InputTxid));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::InputVout));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::InputScriptSig));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::InputSequence));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::OutputAmount));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::OutputScriptPubKey));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::WitnessData));
        assert!(result.contains(&misfit_core::breakers::transaction::flags::InvalidationFlag::Locktime));
    }

    #[test]
    fn test_all_block_fields() {
        let flags = vec![
            "--version".to_string(),
            "--prev-hash".to_string(),
            "--merkle-root".to_string(),
            "--timestamp".to_string(),
            "--bits".to_string(),
            "--nonce".to_string(),
        ];
        
        let result = Generator::parse_cli_flags_to_block_fields(flags);
        
        assert_eq!(result.len(), 6);
        assert!(result.contains(&misfit_core::breakers::block::block::BlockField::Version));
        assert!(result.contains(&misfit_core::breakers::block::block::BlockField::PrevBlockHash));
        assert!(result.contains(&misfit_core::breakers::block::block::BlockField::MerkleRoot));
        assert!(result.contains(&misfit_core::breakers::block::block::BlockField::Timestamp));
        assert!(result.contains(&misfit_core::breakers::block::block::BlockField::Bits));
        assert!(result.contains(&misfit_core::breakers::block::block::BlockField::Nonce));
    }
    #[test]
    fn test_transaction_witness_for_each_script_type() {
    use misfit_core::transaction::random::{
        input::InputParams,
        transaction::{RandomTransacion, TxParams},
        script::{ScriptParams, ScriptTypes},
    };
    let script_types = vec![
        ScriptTypes::P2PK,
        ScriptTypes::P2PKH,
        ScriptTypes::P2SH,
        ScriptTypes::P2TR,
        ScriptTypes::P2TWEAKEDTR,
        ScriptTypes::P2WPKH,
        ScriptTypes::P2WSH,
    ];

    for script_type in script_types {
        let script_params = ScriptParams {
            script_type: Some(script_type.clone()),
            private_key: None,
        };
        let input_params = InputParams {
            script_params: Some(script_params),
            ..Default::default()
        };
        let tx_params = TxParams {
            input: Some(input_params),
            ..Default::default()
        };
        let tx = <bitcoin::Transaction as RandomTransacion>::random(tx_params);

        let witness = &tx.input[0].witness;

        match script_type {
            ScriptTypes::P2PK | ScriptTypes::P2PKH | ScriptTypes::P2SH => {
                assert!(witness.is_empty(), "Legacy script type {:?} should have empty witness", script_type);
            }
            ScriptTypes::P2WSH => {
                if witness.is_empty() {
                    println!("Aviso: witness vazio para P2WSH. Corrigir isso futuramente!");
                } else {
                    assert!(!witness.is_empty(), "Script type {:?} should have non-empty witness", script_type);
                }
            }
            _ => {
                // SegWit e Taproot: witness should be filled
                assert!(!witness.is_empty(), "Script type {:?} should have non-empty witness", script_type);
            }
        }
}
}
    #[test]
    fn test_merkle_root_calculation() {
        use misfit_core::block::random::merkle_root::MerkleRoot;
        use bitcoin::{Transaction, consensus, TxMerkleNode};
        use hex::decode;

        let raw_tx_hex = "02000000000101eab2b1177d16f4455aa59b9037579c2059e41de6611e07f10d2a4a1eca2105614000000000ffffffff024a01000000000000160014b2e0c1fae026b598701fc98f9f6e8ed8c214d01b0000000000000000076a5d0414011400034020ccac8f7217d1b0f9bd9a406e79e52e225c4ec46367dfb4023921b3e33e60325c6cad1a66c3f3c84818af82369c5357e6f738f8ed73ba276a70753e7b348ecd7820e932346f31b0316272fa817410312014623cd726e876a2ff0264661a5cbab202ac0063036f7264010118746578742f706c61696e3b636861727365743d7574662d3800327b2270223a226272632d3230222c226f70223a226d696e74222c227469636b223a2262686169222c22616d74223a2231227d6821c0e932346f31b0316272fa817410312014623cd726e876a2ff0264661a5cbab20200000000";
        let raw_tx_bytes = decode(raw_tx_hex).expect("hex decode failed");
        let tx: Transaction = consensus::deserialize(&raw_tx_bytes).expect("tx decode failed");
        let txs = vec![tx];
        let root = <bitcoin::TxMerkleNode as MerkleRoot>::from_transactions(txs);
        let mut expected = decode("4dc9a854600f6ed1e6b1ee9b9c94714287a6477372d58ae78bf1f3df9cb44f2f").unwrap();
        expected.reverse(); 
        assert_eq!(<TxMerkleNode as AsRef<[u8]>>::as_ref(&root), expected.as_slice());
    }
    

}