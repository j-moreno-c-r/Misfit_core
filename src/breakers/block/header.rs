use super::{bits::BitsProcessor, merkle_root::MerkleRootProcessor, version::VersionProcessor};
use bitcoin::blockdata::block::BlockHash;
use bitcoin::{
    blockdata::block::{Header, Version},
    hashes::Hash,
    pow::CompactTarget,
};
/// Processor for block header modifications
pub struct HeaderProcessor;

impl HeaderProcessor {
    pub fn process_prev_block_hash(_hash: &BlockHash, randomize_hashes: bool) -> BlockHash {
        if randomize_hashes {
            Self::generate_random_block_hash()
        } else {
            BlockHash::all_zeros()
        }
    }

    /// Process the timestamp
    pub fn process_timestamp(timestamp: u32, timestamp_offset: Option<i64>) -> u32 {
        if let Some(offset) = timestamp_offset {
            (timestamp as i64 + offset).max(0) as u32
        } else {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;
            current_time.saturating_add(31_536_000)
        }
    }

    /// Process the nonce
    pub fn process_nonce(nonce: u32) -> u32 {
        // Bitwise NOT to invert all bits

        !nonce
    }

    /// Generate a random block hash
    pub fn generate_random_block_hash() -> BlockHash {
        use rand::Rng;
        let mut rng = rand::rng();
        let random_bytes: [u8; 32] = std::array::from_fn(|_| rng.random());
        BlockHash::from_slice(&random_bytes).expect("Failed to create BlockHash from random bytes")
    }

    /// Generate a random timestamp within reasonable bounds
    pub fn generate_random_timestamp() -> u32 {
        use rand::Rng;
        let mut rng = rand::rng();
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        // Generate timestamp within +/- 10 years of current time
        let ten_years = 10 * 365 * 24 * 60 * 60; // 10 years in seconds
        let min_time = current_time.saturating_sub(ten_years);
        let max_time = current_time.saturating_add(ten_years);

        rng.random_range(min_time..=max_time)
    }

    /// Generate a random nonce
    pub fn generate_random_nonce() -> u32 {
        use rand::Rng;
        let mut rng = rand::rng();
        rng.random()
    }

    /// Validate timestamp (basic sanity check)
    pub fn is_valid_timestamp(timestamp: u32) -> bool {
        // Should be after Bitcoin genesis (January 3, 2009) and before far future
        let genesis_time = 1231006505; // Bitcoin genesis block timestamp
        let far_future = 2147483647; // Year 2038 (max u32 timestamp)

        timestamp >= genesis_time && timestamp <= far_future
    }

    pub fn process_header(
        header: &Header,
        version_override: Option<i32>,
        timestamp_offset: Option<i64>,
        randomize_hashes: bool,
        fields_to_modify: &[super::block::BlockField],
    ) -> Header {
        use super::block::BlockField;

        let mut modified_header = *header;
        let should_modify_all = fields_to_modify.contains(&BlockField::All);

        if should_modify_all || fields_to_modify.contains(&BlockField::Version) {
            let new_version =
                VersionProcessor::process_version(header.version.to_consensus(), version_override);
            modified_header.version = Version::from_consensus(new_version);
        }

        if should_modify_all || fields_to_modify.contains(&BlockField::PrevBlockHash) {
            modified_header.prev_blockhash =
                Self::process_prev_block_hash(&header.prev_blockhash, randomize_hashes);
        }

        if should_modify_all || fields_to_modify.contains(&BlockField::MerkleRoot) {
            modified_header.merkle_root =
                MerkleRootProcessor::process_merkle_root(&header.merkle_root, randomize_hashes);
        }

        if should_modify_all || fields_to_modify.contains(&BlockField::Timestamp) {
            modified_header.time = Self::process_timestamp(header.time, timestamp_offset);
        }

        if should_modify_all || fields_to_modify.contains(&BlockField::Bits) {
            let new_bits = BitsProcessor::process_bits(header.bits.to_consensus());
            modified_header.bits = CompactTarget::from_consensus(new_bits);
        }

        if should_modify_all || fields_to_modify.contains(&BlockField::Nonce) {
            modified_header.nonce = Self::process_nonce(header.nonce);
        }

        modified_header
    }
}
