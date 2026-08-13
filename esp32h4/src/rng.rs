#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: DATA,
    cfg: CFG,
    data_sync: DATA_SYNC,
    _reserved3: [u8; 0x03f0],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - RNG result register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x04 - configure rng register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0x08 - rng result sync register"]
    #[inline(always)]
    pub const fn data_sync(&self) -> &DATA_SYNC {
        &self.data_sync
    }
    #[doc = "0x3fc - Date register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "DATA (r) register accessor: RNG result register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "RNG result register"]
pub mod data;
#[doc = "CFG (rw) register accessor: configure rng register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "configure rng register"]
pub mod cfg;
#[doc = "DATA_SYNC (r) register accessor: rng result sync register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_sync::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_sync`] module"]
pub type DATA_SYNC = crate::Reg<data_sync::DATA_SYNC_SPEC>;
#[doc = "rng result sync register"]
pub mod data_sync;
pub use crate::dma::{date, DATE};
