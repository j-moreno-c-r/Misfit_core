use bitcoin::block::Version;
use misfit_core::block::random::version::RandomVersion;

#[test]
fn test_random_version() {
    Version::random();
}
