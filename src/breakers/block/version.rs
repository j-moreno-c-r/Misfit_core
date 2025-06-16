use bitcoin::blockdata::block::Version;

pub struct VersionProcessor;

impl VersionProcessor {
    /// Process the version of the block with optional override
    pub fn process_version(_version: i32, version_override: Option<i32>) -> i32 {
        if let Some(override_version) = version_override {
            override_version
        } else {
            // Default behavior: set version to maximum valid value
            
            0x3FFFFFFF
        }
    }

    /// Convert consensus version to Bitcoin Version type
    pub fn to_version_type(consensus_version: i32) -> Version {
        Version::from_consensus(consensus_version)
    }

    /// Extract consensus version from Bitcoin Version type
    pub fn from_version_type(version: &Version) -> i32 {
        version.to_consensus()
    }

    /// Validate version value
    pub fn is_valid_version(version: i32) -> bool {
        // Bitcoin version should be positive and within reasonable bounds
        version > 0 && version <= 0x3FFFFFFF
    }

    /// Generate a random valid version
    pub fn generate_random_version() -> i32 {
        use rand::Rng;
        let mut rng = rand::rng();
        rng.random_range(1..=0x3FFFFFFF)
    }

    /// Increment version by a specific amount
    pub fn increment_version(version: i32, increment: i32) -> i32 {
        (version.saturating_add(increment)).min(0x3FFFFFFF)
    }
}