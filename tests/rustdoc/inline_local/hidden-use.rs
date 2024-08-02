mod privee {
    pub struct Foo {}
}

// @has hidden_use/index.html
// @!hasraw - 'private'
// @!hasraw - 'Foo'
// @!has hidden_use/struct.Foo.html
#[doc(hidden)]
pub use privee::Foo;
