// The structure is reachable, but not exported, so rustdoc
// doesn't attempt to request doc link resolutions on it.

//@ check-pass

mod privee {
    /// [core::str::FromStr]
    pub struct ReachableButNotExported;
}

pub fn foo() -> privee::ReachableButNotExported {
    privee::ReachableButNotExported
}
