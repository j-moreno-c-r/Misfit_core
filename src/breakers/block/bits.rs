use bitcoin::pow::CompactTarget;

/// Processor for difficulty bits field modifications
pub struct BitsProcessor;

impl BitsProcessor {
    /// Process the bits (difficulty target)
    pub fn process_bits(bits: u32) -> u32 {
        // XOR with mask to modify difficulty

        bits ^ 0x00FFFFFF
    }

    /// Convert consensus bits to CompactTarget type
    pub fn to_compact_target(consensus_bits: u32) -> CompactTarget {
        CompactTarget::from_consensus(consensus_bits)
    }

    /// Extract consensus bits from CompactTarget type
    pub fn from_compact_target(target: &CompactTarget) -> u32 {
        target.to_consensus()
    }

    /// Generate minimum difficulty bits (easiest target)
    pub fn generate_min_difficulty_bits() -> u32 {
        0x207fffff // Minimum difficulty for Bitcoin
    }

    /// Generate maximum difficulty bits (hardest target)
    pub fn generate_max_difficulty_bits() -> u32 {
        0x1d00ffff // Close to maximum difficulty
    }

    /// Generate random difficulty bits
    pub fn generate_random_bits() -> u32 {
        use rand::Rng;
        let mut rng = rand::rng();

        // Generate within reasonable difficulty range
        let exponent = rng.random_range(0x1d..=0x20); // Reasonable exponent range
        let mantissa = rng.random_range(0x008000..=0xffffff); // Valid mantissa range

        (exponent << 24) | mantissa
    }

    /// Increment difficulty (make mining harder)
    pub fn increase_difficulty(bits: u32, factor: f64) -> u32 {
        let target = Self::bits_to_target(bits);
        let new_target = (target as f64 / factor) as u32;
        Self::target_to_bits(new_target.max(1)) // Ensure non-zero
    }

    /// Decrease difficulty (make mining easier)
    pub fn decrease_difficulty(bits: u32, factor: f64) -> u32 {
        let target = Self::bits_to_target(bits);
        let new_target = (target as f64 * factor) as u32;
        Self::target_to_bits(new_target)
    }

    /// Convert compact bits representation to target value (simplified)
    pub fn bits_to_target(bits: u32) -> u32 {
        let exponent = (bits >> 24) & 0xff;
        let mantissa = bits & 0xffffff;

        if exponent <= 3 {
            mantissa >> (8 * (3 - exponent))
        } else {
            mantissa << (8 * (exponent - 3))
        }
    }

    /// Convert target value to compact bits representation (simplified)
    pub fn target_to_bits(target: u32) -> u32 {
        if target == 0 {
            return 0;
        }

        let mut size = (32 - target.leading_zeros()) as u8;
        let mut compact = if size <= 3 {
            target << (8 * (3 - size as u32))
        } else {
            target >> (8 * (size as u32 - 3))
        };

        // If the sign bit is set, we need to adjust
        if (compact & 0x00800000) != 0 {
            compact >>= 8;
            size += 1;
        }

        ((size as u32) << 24) | compact
    }

    /// Validate bits format
    pub fn is_valid_bits(bits: u32) -> bool {
        let exponent = (bits >> 24) & 0xff;
        let mantissa = bits & 0xffffff;

        // Basic validation: exponent should be reasonable, mantissa non-zero for valid target
        (0x03..=0x20).contains(&exponent) && mantissa != 0
    }

    /// Check if bits represent minimum difficulty
    pub fn is_min_difficulty(bits: u32) -> bool {
        bits >= 0x207fffff
    }

    /// Flip specific bits in the difficulty target
    pub fn flip_bits_pattern(bits: u32, pattern: u32) -> u32 {
        bits ^ pattern
    }

    /// Set bits to a specific difficulty level
    pub fn set_difficulty_level(level: DifficultyLevel) -> u32 {
        match level {
            DifficultyLevel::VeryEasy => 0x207fffff,
            DifficultyLevel::Easy => 0x1f0fffff,
            DifficultyLevel::Medium => 0x1e0fffff,
            DifficultyLevel::Hard => 0x1d0fffff,
            DifficultyLevel::VeryHard => 0x1c0fffff,
        }
    }
}

/// Difficulty levels for testing
#[derive(Debug, Clone, Copy)]
pub enum DifficultyLevel {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    VeryHard,
}
