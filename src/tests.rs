
#[cfg(test)]
mod tests {
    pub use crate::api::{Generator};

    #[test]
    fn test_generate_single_transaction() {
        let result = Generator::transaction(1);
        assert!(result.contains("Raw Transactions:"));
        assert!(result.contains("TXIDs:"));
        assert!(result.contains("---"));
        
        // Split by separator and check structure
        let sections: Vec<&str> = result.split("\n---\n").collect();
        assert_eq!(sections.len(), 2);
        
        // Verify sections have content
        assert!(sections[0].starts_with("Raw Transactions:"));
        assert!(sections[1].starts_with("TXIDs:"));
    }

    #[test]
    fn test_generate_multiple_transactions() {
        let tx_count = 3;
        let result = Generator::transaction(tx_count);
        
        // Check that the result contains expected sections
        assert!(result.contains("Raw Transactions:"));
        assert!(result.contains("TXIDs:"));
        
        // Count occurrences to verify we have the right number of transactions
        // This is a basic check - the exact format depends on the debug implementation
        let raw_tx_section = result.split("TXIDs:").next().unwrap();
        let txid_section = result.split("TXIDs:").nth(1).unwrap();
        
        // Both sections should contain content
        assert!(!raw_tx_section.trim().is_empty());
        assert!(!txid_section.trim().is_empty());
    }

    #[test]
    fn test_generate_zero_transactions() {
        let result = Generator::transaction(0);
        
        // Should still have structure but with empty arrays
        assert!(result.contains("Raw Transactions:"));
        assert!(result.contains("TXIDs:"));
    }

    #[test]
    fn test_generate_single_block() {
        let result = Generator::block(1);
        // Split by separator and check structure
        let sections: Vec<&str> = result.split("\n---\n").collect();
        assert_eq!(sections.len(), 3);
        assert!(result.contains("Header"));
        assert!(result.contains("Raw txs:"));
        assert!(result.contains("TxID:"));
        
    }

    #[test]
    fn test_generate_block_with_multiple_transactions() {
        let tx_count = 2;
        let result = Generator::block(tx_count);
        
        // Check basic structure
        assert!(result.contains("Raw txs:"));
        assert!(result.contains("TxID:"));
        
        let sections: Vec<&str> = result.split("\n---\n").collect();
        assert_eq!(sections.len(), 3);
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
        assert_eq!(result.randomize_hashes, true); // default
    }

    #[test]
    fn test_regtest_invocation() {
        let wallet_name = "test_wallet";
        let cli_mode = "test_mode";
        
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
        let raw_tx = "dummy_tx".to_string();
        let cli_flags = vec![];
        
        let result = Generator::break_transaction(raw_tx, cli_flags);
        
        assert!(result.contains("No invalidation flags specified"));
        assert!(result.contains("Use 'help' for usage information"));
    }

    #[test]
    fn test_break_block_with_no_flags() {
        let block_header = "dummy_header".to_string();
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
}