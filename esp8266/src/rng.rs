#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG register"]
    pub rng: RNG,
}
#[doc = "rng (r) register accessor: RNG register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rng`] module"]
pub type RNG = crate::Reg<rng::RNG_SPEC>;
#[doc = "RNG register"]
pub mod rng;
