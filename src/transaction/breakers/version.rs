use bitcoin::blockdata::transaction::Version;

pub fn invalidate_version(v: Version) -> Version {
    Version(v.0 + 15)
}