use bitcoin::{hash_types::TxMerkleNode, hashes::Hash};

pub struct MerkleRootProcessor;

impl MerkleRootProcessor {
    /// Process the merkle root
    pub fn process_merkle_root(_root: &TxMerkleNode, randomize_hashes: bool) -> TxMerkleNode {
        if randomize_hashes {
            Self::generate_random_merkle_root()
        } else {
            // Zero out the merkle root
            TxMerkleNode::all_zeros()
        }
    }

    /// Generate a random merkle root
    pub fn generate_random_merkle_root() -> TxMerkleNode {
        use rand::Rng;
        let mut rng = rand::rng();
        let random_bytes: [u8; 32] = std::array::from_fn(|_| rng.random());
        TxMerkleNode::from_slice(&random_bytes).expect("Failed to create TxMerkleNode from random bytes")
    }

    /// Create a zero merkle root
    pub fn create_zero_merkle_root() -> TxMerkleNode {
        TxMerkleNode::all_zeros()
    }

    /// Create merkle root from hex string
    pub fn from_hex(hex_str: &str) -> Result<TxMerkleNode, bitcoin::hashes::hex::HexToArrayError> {
        use bitcoin::hashes::hex::FromHex;
        let bytes = <[u8; 32]>::from_hex(hex_str)?;
        Ok(TxMerkleNode::from_byte_array(bytes))
    }

    /// Convert merkle root to hex string
    pub fn to_hex(root: &TxMerkleNode) -> String {
        root.to_string()
    }

    /// XOR two merkle roots
    pub fn xor_merkle_roots(root1: &TxMerkleNode, root2: &TxMerkleNode) -> TxMerkleNode {
        let bytes1 = root1.as_byte_array();
        let bytes2 = root2.as_byte_array();
        
        let mut result_bytes = [0u8; 32];
        for i in 0..32 {
            result_bytes[i] = bytes1[i] ^ bytes2[i];
        }
        
        TxMerkleNode::from_byte_array(result_bytes)
    }

    /// Flip specific bits in merkle root
    pub fn flip_bits(root: &TxMerkleNode, bit_positions: &[usize]) -> TxMerkleNode {
        let mut bytes = *root.as_byte_array();
        
        for &bit_pos in bit_positions {
            if bit_pos < 256 { // 32 bytes * 8 bits = 256 bits
                let byte_index = bit_pos / 8;
                let bit_index = bit_pos % 8;
                bytes[byte_index] ^= 1 << bit_index;
            }
        }
        
        TxMerkleNode::from_byte_array(bytes)
    }

    /// Increment merkle root by 1 (treating as big integer)
    pub fn increment_merkle_root(root: &TxMerkleNode) -> TxMerkleNode {
        let mut bytes = *root.as_byte_array();
        let mut carry = 1u8;
        
        // Add 1 starting from the least significant byte (rightmost)
        for byte in bytes.iter_mut().rev() {
            let sum = *byte as u16 + carry as u16;
            *byte = sum as u8;
            carry = (sum >> 8) as u8;
            
            if carry == 0 {
                break;
            }
        }
        
        TxMerkleNode::from_byte_array(bytes)
    }

    /// Validate merkle root format (basic check)
    pub fn is_valid_format(root: &TxMerkleNode) -> bool {
        // All merkle roots are technically valid 32-byte hashes
        // This is more of a placeholder for future validation logic
        root.as_byte_array().len() == 32
    }
}