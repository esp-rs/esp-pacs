#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    rng: RNG,
}
impl RegisterBlock {
    #[doc = "0x00 - RNG register"]
    #[inline(always)]
    pub const fn rng(&self) -> &RNG {
        &self.rng
    }
}
#[doc = "rng (r) register accessor: RNG register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng`] module"]
pub type RNG = crate::Reg<rng::RNG_SPEC>;
#[doc = "RNG register"]
pub mod rng;
