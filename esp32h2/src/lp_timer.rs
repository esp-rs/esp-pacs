#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub tar0_low: TAR0_LOW,
    #[doc = "0x04 - need_des"]
    pub tar0_high: TAR0_HIGH,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - need_des"]
    pub update: UPDATE,
    #[doc = "0x14 - need_des"]
    pub main_buf0_low: MAIN_BUF0_LOW,
    #[doc = "0x18 - need_des"]
    pub main_buf0_high: MAIN_BUF0_HIGH,
    #[doc = "0x1c - need_des"]
    pub main_buf1_low: MAIN_BUF1_LOW,
    #[doc = "0x20 - need_des"]
    pub main_buf1_high: MAIN_BUF1_HIGH,
    #[doc = "0x24 - need_des"]
    pub main_overflow: MAIN_OVERFLOW,
    #[doc = "0x28 - need_des"]
    pub int_raw: INT_RAW,
    #[doc = "0x2c - need_des"]
    pub int_st: INT_ST,
    #[doc = "0x30 - need_des"]
    pub int_ena: INT_ENA,
    #[doc = "0x34 - need_des"]
    pub int_clr: INT_CLR,
    _reserved12: [u8; 0x03c4],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "TAR0_LOW (rw) register accessor: an alias for `Reg<TAR0_LOW_SPEC>`"]
pub type TAR0_LOW = crate::Reg<tar0_low::TAR0_LOW_SPEC>;
#[doc = "need_des"]
pub mod tar0_low;
#[doc = "TAR0_HIGH (rw) register accessor: an alias for `Reg<TAR0_HIGH_SPEC>`"]
pub type TAR0_HIGH = crate::Reg<tar0_high::TAR0_HIGH_SPEC>;
#[doc = "need_des"]
pub mod tar0_high;
#[doc = "UPDATE (rw) register accessor: an alias for `Reg<UPDATE_SPEC>`"]
pub type UPDATE = crate::Reg<update::UPDATE_SPEC>;
#[doc = "need_des"]
pub mod update;
#[doc = "MAIN_BUF0_LOW (r) register accessor: an alias for `Reg<MAIN_BUF0_LOW_SPEC>`"]
pub type MAIN_BUF0_LOW = crate::Reg<main_buf0_low::MAIN_BUF0_LOW_SPEC>;
#[doc = "need_des"]
pub mod main_buf0_low;
#[doc = "MAIN_BUF0_HIGH (r) register accessor: an alias for `Reg<MAIN_BUF0_HIGH_SPEC>`"]
pub type MAIN_BUF0_HIGH = crate::Reg<main_buf0_high::MAIN_BUF0_HIGH_SPEC>;
#[doc = "need_des"]
pub mod main_buf0_high;
#[doc = "MAIN_BUF1_LOW (r) register accessor: an alias for `Reg<MAIN_BUF1_LOW_SPEC>`"]
pub type MAIN_BUF1_LOW = crate::Reg<main_buf1_low::MAIN_BUF1_LOW_SPEC>;
#[doc = "need_des"]
pub mod main_buf1_low;
#[doc = "MAIN_BUF1_HIGH (r) register accessor: an alias for `Reg<MAIN_BUF1_HIGH_SPEC>`"]
pub type MAIN_BUF1_HIGH = crate::Reg<main_buf1_high::MAIN_BUF1_HIGH_SPEC>;
#[doc = "need_des"]
pub mod main_buf1_high;
#[doc = "MAIN_OVERFLOW (w) register accessor: an alias for `Reg<MAIN_OVERFLOW_SPEC>`"]
pub type MAIN_OVERFLOW = crate::Reg<main_overflow::MAIN_OVERFLOW_SPEC>;
#[doc = "need_des"]
pub mod main_overflow;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
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
