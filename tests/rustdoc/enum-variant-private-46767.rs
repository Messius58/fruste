// https://github.com/rust-lang/rust/issues/46767
#![crate_name = "foo"]

mod privee {
    pub enum Enum{Variant}
}
pub use self::privee::Enum::*;

// @!has-dir foo/private
// @!has foo/index.html '//a/@href' 'private/index.html'
