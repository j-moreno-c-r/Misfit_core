use bitcoin::{
    blockdata::block::{Block, Header},
    consensus::Decodable,
};

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
        println!("\n=== {} ===", label);
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