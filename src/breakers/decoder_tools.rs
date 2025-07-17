use bitcoin::{
    consensus::deserialize,
    Transaction,
    blockdata::block::{Block, Header},
};
use bitcoin::consensus::Decodable;

#[derive(Default)]
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


/* 
    /// Pretty print a decoded transaction
    pub fn print_transaction(&self, decoded: &DecodedTransaction) {
        println!("=== Bitcoin Transaction Details ===");
        println!("TXID: {}", decoded.txid);
        println!("Version: {}", decoded.version);
        println!("Lock Time: {}", decoded.lock_time);
        println!("Size: {} bytes", decoded.size);
        println!("Virtual Size: {} vbytes", decoded.vsize);
        println!("Weight: {} WU", decoded.weight);
        println!();

        println!("INPUTS ({}):", decoded.inputs.len());
        for (i, input) in decoded.inputs.iter().enumerate() {
            println!("  Input {}:", i);
            println!("    Previous Output: {}:{}", input.previous_output.txid, input.previous_output.vout);
            println!("    Script Sig: {}", input.script_sig);
            println!("    Sequence: 0x{:08x}", input.sequence);
            
            if !input.witness.is_empty() {
                println!("    Witness ({} items):", input.witness.len());
                for (j, witness_item) in input.witness.iter().enumerate() {
                    println!("      {}: {}", j, hex::encode(witness_item));
                }
            }
            println!();
        }

        println!("OUTPUTS ({}):", decoded.outputs.len());
        for (i, output) in decoded.outputs.iter().enumerate() {
            println!("  Output {}:", i);
            println!("    Value: {} satoshis ({} BTC)", output.value, output.value as f64 / 100_000_000.0);
            println!("    Script PubKey: {}", output.script_pubkey);
            println!();
        }
    }
*/





// Decoding, utilities, and helper functions implementation
pub struct BlockUtils;
impl BlockUtils {
    // Utility method to decode block header from hex string
    pub fn decode_header_from_hex(hex_string: &str) -> Result<Header, Box<dyn std::error::Error>> {
        let bytes = hex::decode(hex_string)?;
        if bytes.len() != 80 {
            return Err(format!("Invalid header length: expected 80 bytes, got {}", bytes.len()).into());
        }
        let header = Header::consensus_decode(&mut &bytes[..])?;
        Ok(header)
    }

    // Utility method to decode block from hex string
    pub fn decode_block_from_hex(hex_string: &str) -> Result<Block, Box<dyn std::error::Error>> {
        let bytes = hex::decode(hex_string)?;
        let block = Block::consensus_decode(&mut &bytes[..])?;
        Ok(block)
    }

    // Create a minimal block from a header (for testing purposes)
    pub fn create_minimal_block_from_header(header: Header) -> Block {
        Block {
            header,
            txdata: vec![], // Empty transaction list
        }
    }

    // Print block header information
    pub fn print_header_info(header: &Header, label: &str) {
        println!("\n=== {label} ===");
        println!("Version: {}", header.version.to_consensus());
        println!("Previous Block: {}", header.prev_blockhash);
        println!("Merkle Root: {}", header.merkle_root);
        println!("Timestamp: {}", header.time);
        println!("Bits: 0x{:08x}", header.bits.to_consensus());
        println!("Nonce: {}", header.nonce);
        println!("Block Hash: {}", header.block_hash());
    }

    // Encode header to hex string
    pub fn encode_header_to_hex(header: &Header) -> String {
        use bitcoin::consensus::Encodable;
        let mut bytes = Vec::new();
        header.consensus_encode(&mut bytes).expect("Failed to encode header");
        hex::encode(bytes)
    }


}

