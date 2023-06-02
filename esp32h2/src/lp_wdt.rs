#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub wdtconfig0: WDTCONFIG0,
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
    pub wdtfeed: WDTFEED,
    #[doc = "0x1c - need_des"]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0x20 - need_des"]
    pub swd_conf: SWD_CONF,
    #[doc = "0x24 - need_des"]
    pub swd_wprotect: SWD_WPROTECT,
    #[doc = "0x28 - need_des"]
    pub int_raw: INT_RAW,
    #[doc = "0x2c - need_des"]
    pub int_st_rtc: INT_ST_RTC,
    #[doc = "0x30 - need_des"]
    pub int_ena_rtc: INT_ENA_RTC,
    #[doc = "0x34 - need_des"]
    pub int_clr_rtc: INT_CLR_RTC,
    _reserved14: [u8; 0x03c4],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "WDTCONFIG0 (rw) register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "need_des"]
pub mod wdtconfig0;
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
#[doc = "WDTFEED (w) register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "need_des"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "need_des"]
pub mod wdtwprotect;
#[doc = "SWD_CONF (rw) register accessor: an alias for `Reg<SWD_CONF_SPEC>`"]
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
#[doc = "need_des"]
pub mod swd_conf;
#[doc = "SWD_WPROTECT (rw) register accessor: an alias for `Reg<SWD_WPROTECT_SPEC>`"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "need_des"]
pub mod swd_wprotect;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod int_raw;
#[doc = "INT_ST_RTC (r) register accessor: an alias for `Reg<INT_ST_RTC_SPEC>`"]
pub type INT_ST_RTC = crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>;
#[doc = "need_des"]
pub mod int_st_rtc;
#[doc = "INT_ENA_RTC (rw) register accessor: an alias for `Reg<INT_ENA_RTC_SPEC>`"]
pub type INT_ENA_RTC = crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>;
#[doc = "need_des"]
pub mod int_ena_rtc;
#[doc = "INT_CLR_RTC (w) register accessor: an alias for `Reg<INT_CLR_RTC_SPEC>`"]
pub type INT_CLR_RTC = crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>;
#[doc = "need_des"]
pub mod int_clr_rtc;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
