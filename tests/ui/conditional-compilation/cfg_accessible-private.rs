//@ check-pass

#![feature(cfg_accessible)]

mod privee {
    struct Struct;
    enum Enum{}
    union Union{_a:u8}
}

#[cfg_accessible(privee::Struct)]
const A: bool = true;

#[cfg_accessible(privee::Enum)]
const A: bool = true;

#[cfg_accessible(privee::Union)]
const A: bool = true;

const A: bool = false; // Will conflict if any of those is accessible
fn main() {}
