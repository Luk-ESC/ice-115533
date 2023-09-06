#![feature(const_trait_impl)]
#![feature(effects)]

#[const_trait]
pub trait ComponentId {
    fn x();
}
