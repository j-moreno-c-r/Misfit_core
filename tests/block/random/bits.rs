use bitcoin::CompactTarget;
use misfit_core::block::random::bits::RandomBits;

#[test]
fn test_random_bits() {
    let bits: CompactTarget = CompactTarget::random();
    assert!(bits.to_consensus() >= 1)
}
