use bitcoin::CompactTarget;
use secp256k1::rand::{self, Rng};

pub trait RandomBits {
    fn random() -> CompactTarget;
}

impl RandomBits for CompactTarget {
    fn random() -> CompactTarget {
        CompactTarget::from_consensus(rand::thread_rng().gen::<u32>())
    }
}
