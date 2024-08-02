pub use hidden::Bar;
pub use privee::Quz;

mod privee {
    pub struct Quz;
}

#[doc(hidden)]
pub mod hidden {
    pub struct Bar;
}

#[macro_export]
macro_rules! foo {
    () => {};
}
