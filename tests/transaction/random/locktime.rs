use bitcoin::absolute::LockTime;
use misfit_core::transaction::random::locktime::RandomLockTime;

#[test]
fn test_random_lock_time() {
    let locktime: LockTime = LockTime::random();

    assert!(locktime.is_block_height() || locktime.is_block_time())
}