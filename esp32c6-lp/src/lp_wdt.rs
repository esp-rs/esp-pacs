#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub config0: CONFIG0,
    #[doc = "0x04 - need_des"]
    pub config1: CONFIG1,
    #[doc = "0x08 - need_des"]
    pub config2: CONFIG2,
    #[doc = "0x0c - need_des"]
    pub config3: CONFIG3,
    #[doc = "0x10 - need_des"]
    pub config4: CONFIG4,
    #[doc = "0x14 - need_des"]
    pub feed: FEED,
    #[doc = "0x18 - need_des"]
    pub wprotect: WPROTECT,
    #[doc = "0x1c - need_des"]
    pub swd_config: SWD_CONFIG,
    #[doc = "0x20 - need_des"]
    pub swd_wprotect: SWD_WPROTECT,
    #[doc = "0x24 - need_des"]
    pub int_raw: INT_RAW,
    #[doc = "0x28 - need_des"]
    pub int_st: INT_ST,
    #[doc = "0x2c - need_des"]
    pub int_ena: INT_ENA,
    #[doc = "0x30 - need_des"]
    pub int_clr: INT_CLR,
    _reserved13: [u8; 0x03c8],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "CONFIG0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`config0`] module"]
pub type CONFIG0 = crate::Reg<config0::CONFIG0_SPEC>;
#[doc = "need_des"]
pub mod config0;
#[doc = "CONFIG1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`config1`] module"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = "need_des"]
pub mod config1;
#[doc = "CONFIG2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`config2`] module"]
pub type CONFIG2 = crate::Reg<config2::CONFIG2_SPEC>;
#[doc = "need_des"]
pub mod config2;
#[doc = "CONFIG3 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`config3`] module"]
pub type CONFIG3 = crate::Reg<config3::CONFIG3_SPEC>;
#[doc = "need_des"]
pub mod config3;
#[doc = "CONFIG4 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`config4`] module"]
pub type CONFIG4 = crate::Reg<config4::CONFIG4_SPEC>;
#[doc = "need_des"]
pub mod config4;
#[doc = "FEED (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`feed`] module"]
pub type FEED = crate::Reg<feed::FEED_SPEC>;
#[doc = "need_des"]
pub mod feed;
#[doc = "WPROTECT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wprotect`] module"]
pub type WPROTECT = crate::Reg<wprotect::WPROTECT_SPEC>;
#[doc = "need_des"]
pub mod wprotect;
#[doc = "SWD_CONFIG (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swd_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swd_config`] module"]
pub type SWD_CONFIG = crate::Reg<swd_config::SWD_CONFIG_SPEC>;
#[doc = "need_des"]
pub mod swd_config;
#[doc = "SWD_WPROTECT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swd_wprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_wprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swd_wprotect`] module"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "need_des"]
pub mod swd_wprotect;
#[doc = "INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "need_des"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod int_clr;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
