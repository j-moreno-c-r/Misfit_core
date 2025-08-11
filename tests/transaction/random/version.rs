use bitcoin::transaction::Version;
use misfit_core::transaction::random::version::RandomVersion;

#[test]
fn test_random_version() {
    Version::random();
}