#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    rng_cfg: RNG_CFG,
    rng_data: RNG_DATA,
    rng_rstn: RNG_RSTN,
    rng_date: RNG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - configure rng register"]
    #[inline(always)]
    pub const fn rng_cfg(&self) -> &RNG_CFG {
        &self.rng_cfg
    }
    #[doc = "0x04 - RNG result register"]
    #[inline(always)]
    pub const fn rng_data(&self) -> &RNG_DATA {
        &self.rng_data
    }
    #[doc = "0x08 - rng rstn register"]
    #[inline(always)]
    pub const fn rng_rstn(&self) -> &RNG_RSTN {
        &self.rng_rstn
    }
    #[doc = "0x0c - RNG_DATE"]
    #[inline(always)]
    pub const fn rng_date(&self) -> &RNG_DATE {
        &self.rng_date
    }
}
#[doc = "RNG_CFG (rw) register accessor: configure rng register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cfg`] module"]
pub type RNG_CFG = crate::Reg<rng_cfg::RNG_CFG_SPEC>;
#[doc = "configure rng register"]
pub mod rng_cfg;
#[doc = "RNG_DATA (r) register accessor: RNG result register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_data`] module"]
pub type RNG_DATA = crate::Reg<rng_data::RNG_DATA_SPEC>;
#[doc = "RNG result register"]
pub mod rng_data;
#[doc = "RNG_RSTN (rw) register accessor: rng rstn register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_rstn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_rstn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_rstn`] module"]
pub type RNG_RSTN = crate::Reg<rng_rstn::RNG_RSTN_SPEC>;
#[doc = "rng rstn register"]
pub mod rng_rstn;
#[doc = "RNG_DATE (rw) register accessor: RNG_DATE\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_date`] module"]
pub type RNG_DATE = crate::Reg<rng_date::RNG_DATE_SPEC>;
#[doc = "RNG_DATE"]
pub mod rng_date;
