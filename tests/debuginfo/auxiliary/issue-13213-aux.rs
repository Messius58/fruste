#![crate_type = "lib"]
//@ compile-flags:-g

pub use privee::P;

#[derive(Copy, Clone)]
pub struct S {
    p: P,
}

mod privee {
    #[derive(Copy, Clone)]
    pub struct P {
        p: i32,
    }
    pub const THREE: P = P { p: 3 };
}

pub static A: S = S { p: privee::THREE };
