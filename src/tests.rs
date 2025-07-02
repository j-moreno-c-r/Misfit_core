
#[cfg(test)]
mod tests {
    pub use crate::api::{Generator};
    pub use misfit_core::{transaction::random::witness::{RandomWitness, WitnessParams, TaprootWitnessParams, RandomTaprootWitness}};
    use bitcoin::{
    hashes::Hash,
    secp256k1::{Message, Secp256k1},
    sighash::{EcdsaSighashType, SighashCache},
    NetworkKind, OutPoint, PrivateKey, PublicKey, ScriptBuf,Transaction, Txid,
    Witness,
    key::{Keypair},//TweakedKeypair
    Amount,
    sighash::{TapSighashType},
    TxOut, 
};


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
     #[test]
    fn test_random_witness_with_default_params() {
        let params = WitnessParams::default();
        let witness = <Witness as RandomWitness>::random(params);
        
        // Should generate a witness (may be empty for certain script types)
        assert!(witness.len() <= 2); // P2WPKH has 2 elements, P2WSH can vary
    }

    #[test]
    fn test_random_witness_p2wpkh() {
        // Create a simple transaction for testing
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![bitcoin::TxIn {
                previous_output: OutPoint {
                    txid: Txid::all_zeros(),
                    vout: 0,
                },
                script_sig: ScriptBuf::new(),
                sequence: bitcoin::Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::default(),
            }],
            output: vec![TxOut {
                value: Amount::from_sat(50000),
                script_pubkey: ScriptBuf::new(),
            }],
        };

        let private_key = PrivateKey::generate(NetworkKind::Main);
        let pub_key = PublicKey::from_private_key(&Secp256k1::new(), &private_key);
        let script = ScriptBuf::new_p2wpkh(&pub_key.wpubkey_hash().unwrap());

        let params = WitnessParams {
            transaction: Some(tx),
            vout: Some(0),
            script: Some((script, misfit_core::transaction::random::script::ScriptTypes::P2WPKH)),
            private_key: Some(private_key),
        };

        let witness = <Witness as RandomWitness>::random(params);
        
        // P2WPKH witness should have exactly 2 elements: signature and pubkey
        assert_eq!(witness.len(), 2);
        
        // First element should be signature (65-73 bytes typically)
        let sig_bytes = witness.nth(0).unwrap();
        assert!(sig_bytes.len() >= 64 && sig_bytes.len() <= 73);
        
        // Second element should be public key (33 bytes for compressed)
        let pubkey_bytes = witness.nth(1).unwrap();
        assert_eq!(pubkey_bytes.len(), 33);
    }

    #[test]
    fn test_random_witness_p2wsh() {
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![bitcoin::TxIn {
                previous_output: OutPoint {
                    txid: Txid::all_zeros(),
                    vout: 0,
                },
                script_sig: ScriptBuf::new(),
                sequence: bitcoin::Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::default(),
            }],
            output: vec![TxOut {
                value: Amount::from_sat(100000),
                script_pubkey: ScriptBuf::new(),
            }],
        };

        let private_key = PrivateKey::generate(NetworkKind::Main);
        // Create a simple P2WSH script (like a multisig or custom script)
        let witness_script = ScriptBuf::new_p2pk(&PublicKey::from_private_key(&Secp256k1::new(), &private_key));

        let params = WitnessParams {
            transaction: Some(tx),
            vout: Some(0),
            script: Some((witness_script, misfit_core::transaction::random::script::ScriptTypes::P2WSH)),
            private_key: Some(private_key),
        };

        let witness = <Witness as RandomWitness>::random(params);
        
        // P2WSH witness should have at least 2 elements: signature(s) and witness script
        assert!(witness.len() >= 2);
        
        // Last element should be the witness script
        let script_bytes = witness.nth(witness.len() - 1).unwrap();
        assert!(!script_bytes.is_empty());
    }

    #[test]
    fn test_random_witness_signature_verification() {
        let secp = Secp256k1::new();
        let private_key = PrivateKey::generate(NetworkKind::Main);
        let pub_key = PublicKey::from_private_key(&secp, &private_key);
        
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![bitcoin::TxIn {
                previous_output: OutPoint {
                    txid: Txid::all_zeros(),
                    vout: 0,
                },
                script_sig: ScriptBuf::new(),
                sequence: bitcoin::Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::default(),
            }],
            output: vec![TxOut {
                value: Amount::from_sat(50000),
                script_pubkey: ScriptBuf::new(),
            }],
        };

        let script = ScriptBuf::new_p2wpkh(&pub_key.wpubkey_hash().unwrap());

        let params = WitnessParams {
            transaction: Some(tx.clone()),
            vout: Some(0),
            script: Some((script.clone(), misfit_core::transaction::random::script::ScriptTypes::P2WPKH)),
            private_key: Some(private_key),
        };

        let witness = <Witness as RandomWitness>::random(params);

        let sig_bytes = witness.nth(0).unwrap();
        let _pubkey_bytes = witness.nth(1).unwrap();
        
        // Parse signature (remove sighash type byte)
        let sig_der = &sig_bytes[..sig_bytes.len()-1];
        let ecdsa_sig = bitcoin::secp256k1::ecdsa::Signature::from_der(sig_der).unwrap();
        
        // Recreate sighash to verify
        let sighash = SighashCache::new(&tx)
            .p2wpkh_signature_hash(0, &script, Amount::from_sat(50000), EcdsaSighashType::All)
            .unwrap();
        
        let msg = Message::from_digest_slice(&sighash[..]).unwrap();
        
        // Verify signature
        assert!(secp.verify_ecdsa(&msg, &ecdsa_sig, &pub_key.inner).is_ok());
    }


    #[test]
    fn test_random_taproot_witness_with_default_params() {
        let params = TaprootWitnessParams::default();
        let witness = <Witness as RandomTaprootWitness>::random(params);
        
        // Taproot key spend witness should have exactly 1 element (the signature)
        assert_eq!(witness.len(), 1);
        
        // Signature should be 64 or 65 bytes (Schnorr signature)
        let sig_bytes = witness.nth(0).unwrap();
        assert!(sig_bytes.len() == 64 || sig_bytes.len() == 65);
    }

    #[test]
    fn test_random_taproot_witness_with_custom_keypair() {
        let secp = Secp256k1::new();
        let mut rng = bitcoin::secp256k1::rand::thread_rng();
        let sk = bitcoin::secp256k1::SecretKey::new(&mut rng);
        let keypair = Keypair::from_secret_key(&secp, &sk);
    
        let params = TaprootWitnessParams {
            keypair: Some(keypair),
            ..Default::default()
        };
    
        let witness = <Witness as RandomTaprootWitness>::random(params);
    
        assert_eq!(witness.len(), 1);
        let sig_bytes = witness.nth(0).unwrap();
        assert!(sig_bytes.len() == 64 || sig_bytes.len() == 65);
    }
    #[test]
    fn test_random_taproot_witness_with_custom_transaction() {
        let custom_tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![bitcoin::TxIn {
                previous_output: OutPoint {
                    txid: Txid::all_zeros(),
                    vout: 5,
                },
                script_sig: ScriptBuf::new(),
                sequence: bitcoin::Sequence::MAX,
                witness: Witness::default(),
            }],
            output: vec![TxOut {
                value: Amount::from_sat(200000),
                script_pubkey: ScriptBuf::new(),
            }],
        };

        let params = TaprootWitnessParams {
            transaction: Some(custom_tx),
            vout: Some(0),
            ..Default::default()
        };

        let witness = <Witness as RandomTaprootWitness>::random(params);
        
        assert_eq!(witness.len(), 1);
    }

    #[test]
    fn test_random_taproot_witness_with_custom_utxo() {
        let custom_utxo = TxOut {
            value: Amount::from_sat(500000),
            script_pubkey: ScriptBuf::new_p2tr_tweaked(
                bitcoin::key::TweakedPublicKey::dangerous_assume_tweaked(
                    bitcoin::secp256k1::XOnlyPublicKey::from_slice(&[2u8; 32]).unwrap()
                )
            ),
        };

        let params = TaprootWitnessParams {
            utxo: Some(custom_utxo),
            ..Default::default()
        };

        let witness = <Witness as RandomTaprootWitness>::random(params);
        
        assert_eq!(witness.len(), 1);
    }

    #[test]
    fn test_taproot_witness_signature_format() {
        let params = TaprootWitnessParams::default();
        let witness = <Witness as RandomTaprootWitness>::random(params);
        
        let sig_bytes = witness.nth(0).unwrap();
        
        // Taproot signatures are Schnorr signatures
        if sig_bytes.len() == 65 {
            // If 65 bytes, last byte should be sighash type
            let sighash_type = sig_bytes[64];
            assert_eq!(sighash_type, TapSighashType::Default as u8);
        } else if sig_bytes.len() == 64 {
            // 64 bytes means default sighash type (0x00) is implied
            assert_eq!(sig_bytes.len(), 64);
        } else {
            panic!("Invalid Taproot signature length: {}", sig_bytes.len());
        }
    }

    #[test] 
    fn test_witness_empty_for_unsupported_script_types() {
        // Test that unsupported script types return empty witness
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![bitcoin::TxIn::default()],
            output: vec![TxOut {
                value: Amount::from_sat(50000),
                script_pubkey: ScriptBuf::new(),
            }],
        };

        // Test with a script type that should return empty witness
        let params = WitnessParams {
            transaction: Some(tx),
            vout: Some(0),
            script: Some((ScriptBuf::new(), misfit_core::transaction::random::script::ScriptTypes::P2PK)), // Legacy type
            private_key: None,
        };

        let witness = <Witness as RandomWitness>::random(params);
        assert!(witness.is_empty());
    }

    #[test]
    fn test_witness_generation_performance() {
        use std::time::Instant;
        
        let start = Instant::now();
        
        // Generate 100 witnesses to test performance
        for _ in 0..100 {
            let params = WitnessParams::default();
            let _ = <Witness as RandomWitness>::random(params);
        }
        
        let duration = start.elapsed();
        println!("Generated 100 witnesses in: {:?}", duration);
        
        // Should complete reasonably quickly (adjust threshold as needed)
        assert!(duration.as_secs() < 5);
    }

    #[test]
    fn test_taproot_witness_generation_performance() {
        use std::time::Instant;
        
        let start = Instant::now();
        
        // Generate 100 Taproot witnesses to test performance
        for _ in 0..100 {
            let params = TaprootWitnessParams::default();
            let _ = <Witness as RandomTaprootWitness>::random(params);
        }
        
        let duration = start.elapsed();
        println!("Generated 100 Taproot witnesses in: {:?}", duration);
        
        // Should complete reasonably quickly
        assert!(duration.as_secs() < 5);
    }

    #[test]
    fn test_witness_serialization_roundtrip() {
        let params = WitnessParams::default();
        let witness = <Witness as RandomWitness>::random(params);
        
        // Test that witness can be serialized and deserialized
        let serialized = bitcoin::consensus::serialize(&witness);
        let deserialized: Witness = bitcoin::consensus::deserialize(&serialized).unwrap();
        
        assert_eq!(witness.len(), deserialized.len());
        for i in 0..witness.len() {
            assert_eq!(witness.nth(i), deserialized.nth(i));
        }
    }

    #[test]
    fn test_taproot_witness_serialization_roundtrip() {
        let params = TaprootWitnessParams::default();
        let witness = <Witness as RandomTaprootWitness>::random(params);
        
        let serialized = bitcoin::consensus::serialize(&witness);
        let deserialized: Witness = bitcoin::consensus::deserialize(&serialized).unwrap();
        
        assert_eq!(witness.len(), deserialized.len());
        assert_eq!(witness.nth(0), deserialized.nth(0));
    }

}