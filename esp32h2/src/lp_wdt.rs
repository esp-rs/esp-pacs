#[doc = r"Register block"]
#[repr(C)]
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
    pub config5: CONFIG5,
    #[doc = "0x18 - need_des"]
    pub feed: FEED,
    #[doc = "0x1c - need_des"]
    pub wprotect: WPROTECT,
    #[doc = "0x20 - need_des"]
    pub swd_config: SWD_CONFIG,
    #[doc = "0x24 - need_des"]
    pub swd_wprotect: SWD_WPROTECT,
    #[doc = "0x28 - need_des"]
    pub int_raw: INT_RAW,
    #[doc = "0x2c - need_des"]
    pub int_st: INT_ST,
    #[doc = "0x30 - need_des"]
    pub int_ena: INT_ENA,
    #[doc = "0x34 - need_des"]
    pub int_clr: INT_CLR,
    _reserved14: [u8; 0x03c4],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "CONFIG0 (rw) register accessor: an alias for `Reg<CONFIG0_SPEC>`"]
pub type CONFIG0 = crate::Reg<config0::CONFIG0_SPEC>;
#[doc = "need_des"]
pub mod config0;
#[doc = "CONFIG1 (rw) register accessor: an alias for `Reg<CONFIG1_SPEC>`"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = "need_des"]
pub mod config1;
#[doc = "CONFIG2 (rw) register accessor: an alias for `Reg<CONFIG2_SPEC>`"]
pub type CONFIG2 = crate::Reg<config2::CONFIG2_SPEC>;
#[doc = "need_des"]
pub mod config2;
#[doc = "CONFIG3 (rw) register accessor: an alias for `Reg<CONFIG3_SPEC>`"]
pub type CONFIG3 = crate::Reg<config3::CONFIG3_SPEC>;
#[doc = "need_des"]
pub mod config3;
#[doc = "CONFIG4 (rw) register accessor: an alias for `Reg<CONFIG4_SPEC>`"]
pub type CONFIG4 = crate::Reg<config4::CONFIG4_SPEC>;
#[doc = "need_des"]
pub mod config4;
#[doc = "CONFIG5 (rw) register accessor: an alias for `Reg<CONFIG5_SPEC>`"]
pub type CONFIG5 = crate::Reg<config5::CONFIG5_SPEC>;
#[doc = "need_des"]
pub mod config5;
#[doc = "FEED (w) register accessor: an alias for `Reg<FEED_SPEC>`"]
pub type FEED = crate::Reg<feed::FEED_SPEC>;
#[doc = "need_des"]
pub mod feed;
#[doc = "WPROTECT (rw) register accessor: an alias for `Reg<WPROTECT_SPEC>`"]
pub type WPROTECT = crate::Reg<wprotect::WPROTECT_SPEC>;
#[doc = "need_des"]
pub mod wprotect;
#[doc = "SWD_CONFIG (rw) register accessor: an alias for `Reg<SWD_CONFIG_SPEC>`"]
pub type SWD_CONFIG = crate::Reg<swd_config::SWD_CONFIG_SPEC>;
#[doc = "need_des"]
pub mod swd_config;
#[doc = "SWD_WPROTECT (rw) register accessor: an alias for `Reg<SWD_WPROTECT_SPEC>`"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "need_des"]
pub mod swd_wprotect;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "need_des"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod int_clr;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
