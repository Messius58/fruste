//@ check-pass

#![deny(rustdoc::private_doc_tests)]

pub fn foo() {}

mod privee {
    /// re-exported doc test
    ///
    /// ```
    /// assert!(true);
    /// ```
    pub fn bar() {}
}

pub use privee::bar;
