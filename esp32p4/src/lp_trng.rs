#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: CFG,
    data: DATA,
    rstn: RSTN,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - configure rng register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0x04 - RNG result register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x08 - rng rstn register"]
    #[inline(always)]
    pub const fn rstn(&self) -> &RSTN {
        &self.rstn
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CFG (rw) register accessor: configure rng register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "configure rng register"]
pub mod cfg;
#[doc = "DATA (r) register accessor: RNG result register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "RNG result register"]
pub mod data;
#[doc = "RSTN (rw) register accessor: rng rstn register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstn`] module"]
pub type RSTN = crate::Reg<rstn::RSTN_SPEC>;
#[doc = "rng rstn register"]
pub mod rstn;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
