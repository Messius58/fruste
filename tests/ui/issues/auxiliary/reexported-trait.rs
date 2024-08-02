mod privee {
    pub trait Trait {
        fn trait_method(&self) {
        }
    }
    pub trait TraitB {
        fn trait_method_b(&self) {
        }
    }
}

pub struct FooStruct;
pub use crate::privee::Trait;
impl crate::privee::Trait for FooStruct {}

pub use crate::privee::TraitB as TraitBRename;
impl crate::privee::TraitB for FooStruct {}
