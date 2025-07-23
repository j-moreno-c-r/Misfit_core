use bitcoin::blockdata::block::{Block, Header};
use super::header::HeaderProcessor;
use super::decoder_tools::BlockUtils;

// Re-export the enum and config from the original design
#[derive(Debug, Clone, PartialEq)]
pub enum BlockField {
    Version,
    PrevBlockHash,
    MerkleRoot,
    Timestamp,
    Bits,
    Nonce,
    All,
}

// Configuration for block processing
#[derive(Debug, Clone, Default)]
pub struct ProcessingConfig {
    pub fields_to_modify: Vec<BlockField>,
    pub version_override: Option<i32>,
    pub timestamp_offset: Option<i64>, // seconds to add/subtract
    pub randomize_hashes: bool,
}



// Block processing and modification implementation
pub struct BlockProcessor {
    config: ProcessingConfig,
}

impl BlockProcessor {
    pub fn new(config: ProcessingConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self {
            config: ProcessingConfig::default(),
        }
    }

    // Check if a specific field should be processed
    fn _should_process_field(&self, field: &BlockField) -> bool {
        self.config.fields_to_modify.contains(&BlockField::All) ||
        self.config.fields_to_modify.contains(field)
    }

    // Process the entire block header based on configuration
    pub fn process_block_header(&self, header: &Header) -> Header {
        HeaderProcessor::process_header(
            header,
            self.config.version_override,
            self.config.timestamp_offset,
            self.config.randomize_hashes,
            &self.config.fields_to_modify,
        )
    }

    // Process an entire block
    pub fn process_block(&self, block: &Block) -> Block {
        let modified_header = self.process_block_header(&block.header);
        
        Block {
            header: modified_header,
            txdata: block.txdata.clone(),
        }
    }

    // Update configuration
    pub fn update_config(&mut self, new_config: ProcessingConfig) {
        self.config = new_config;
    }

    // Get current configuration
    pub fn get_config(&self) -> &ProcessingConfig {
        &self.config
    }

    // Add field to modification list
    pub fn add_field_to_modify(&mut self, field: BlockField) {
        if !self.config.fields_to_modify.contains(&field) {
            self.config.fields_to_modify.push(field);
        }
    }

    // Remove field from modification list
    pub fn remove_field_to_modify(&mut self, field: &BlockField) {
        self.config.fields_to_modify.retain(|f| f != field);
    }

    // Set version override
    pub fn set_version_override(&mut self, version: Option<i32>) {
        self.config.version_override = version;
    }

    // Set timestamp offset
    pub fn set_timestamp_offset(&mut self, offset: Option<i64>) {
        self.config.timestamp_offset = offset;
    }

    // Set hash randomization
    pub fn set_randomize_hashes(&mut self, randomize: bool) {
        self.config.randomize_hashes = randomize;
    }
}

pub struct BlockBreaker;

impl BlockBreaker {
    pub fn break_all_fields(block: &Block) -> Block {
        let processor = BlockProcessor::with_default_config();
        processor.process_block(block)
    }

    pub fn break_specific_fields(block: &Block, fields: Vec<BlockField>) -> Block {
        let config = ProcessingConfig {
            fields_to_modify: fields,
            ..Default::default()
        };
        let processor = BlockProcessor::new(config);
        processor.process_block(block)
    }

    pub fn break_with_config(block: &Block, config: ProcessingConfig) -> Block {
        let processor = BlockProcessor::new(config);
        processor.process_block(block)
    }

    pub fn break_header_fields(header: &Header, fields: Vec<BlockField>) -> Block {
        let config = ProcessingConfig {
            fields_to_modify: fields,
            ..Default::default()
        };
        let processor = BlockProcessor::new(config);
        let modified_header = processor.process_block_header(header);
        BlockUtils::create_minimal_block_from_header(modified_header)
    }

    // Break single field with default settings
    pub fn break_single_field(block: &Block, field: BlockField) -> Block {
        Self::break_specific_fields(block, vec![field])
    }

    // Break multiple fields with custom settings
    pub fn break_fields_with_settings(
        block: &Block, 
        fields: Vec<BlockField>,
        version_override: Option<i32>,
        timestamp_offset: Option<i64>,
        randomize_hashes: bool,
    ) -> Block {
        let config = ProcessingConfig {
            fields_to_modify: fields,
            version_override,
            timestamp_offset,
            randomize_hashes,
        };
        Self::break_with_config(block, config)
    }

    // Create a completely randomized block header
    pub fn create_random_header() -> Header {
        use super::{
            version::VersionProcessor,
            header::HeaderProcessor,
            merkle_root::MerkleRootProcessor,
            bits::BitsProcessor,
        };
        use bitcoin::{blockdata::block::Version, pow::CompactTarget};

        let version = Version::from_consensus(VersionProcessor::generate_random_version());
        let prev_blockhash = HeaderProcessor::generate_random_block_hash();
        let merkle_root = MerkleRootProcessor::generate_random_merkle_root();
        let time = HeaderProcessor::generate_random_timestamp();
        let bits = CompactTarget::from_consensus(BitsProcessor::generate_random_bits());
        let nonce = HeaderProcessor::generate_random_nonce();

        Header {
            version,
            prev_blockhash,
            merkle_root,
            time,
            bits,
            nonce,
        }
    }

    pub fn create_random_block() -> Block {
        let header = Self::create_random_header();
        BlockUtils::create_minimal_block_from_header(header)
    }
}