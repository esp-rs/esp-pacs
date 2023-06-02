#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG register"]
    pub rng: RNG,
}
#[doc = "rng (r) register accessor: an alias for `Reg<RNG_SPEC>`"]
pub type RNG = crate::Reg<rng::RNG_SPEC>;
#[doc = "RNG register"]
pub mod rng;
