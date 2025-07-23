use bitcoin::absolute::LockTime;

pub fn invalidate_locktime(lt: LockTime) -> LockTime {
    match lt {
        LockTime::Blocks(height) => {
            LockTime::Blocks(
                bitcoin::absolute::Height::from_consensus(
                    u32::MAX - height.to_consensus_u32()
                ).unwrap_or(height)
            )
        },
        LockTime::Seconds(time) => {
            LockTime::Seconds(
                bitcoin::absolute::Time::from_consensus(
                    u32::MAX - time.to_consensus_u32()
                ).unwrap_or(time)
            )
        },
    }
}