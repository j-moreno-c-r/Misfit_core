use bitcoin::block::Version;
use misfit_core::block::random::version::RandomVersion;

#[test]
fn test_random_version() {
    let version: Version = Version::random();
    assert!(version.to_consensus() >= 0)
}
