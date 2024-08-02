pub use privee::Quz;
pub use hidden::Bar;

mod privee {
    pub struct Quz;
}

#[doc(hidden)]
pub mod hidden {
    pub struct Bar;
}
