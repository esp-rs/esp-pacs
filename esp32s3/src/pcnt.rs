#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration register 0 for unit %s"]
    pub conf00: CONF0,
    #[doc = "0x04 - Configuration register 1 for unit %s"]
    pub conf10: CONF1,
    #[doc = "0x08 - Configuration register 2 for unit %s"]
    pub conf20: CONF2,
    #[doc = "0x0c - Configuration register 0 for unit %s"]
    pub conf01: CONF0,
    #[doc = "0x10 - Configuration register 1 for unit %s"]
    pub conf11: CONF1,
    #[doc = "0x14 - Configuration register 2 for unit %s"]
    pub conf21: CONF2,
    #[doc = "0x18 - Configuration register 0 for unit %s"]
    pub conf02: CONF0,
    #[doc = "0x1c - Configuration register 1 for unit %s"]
    pub conf12: CONF1,
    #[doc = "0x20 - Configuration register 2 for unit %s"]
    pub conf22: CONF2,
    #[doc = "0x24 - Configuration register 0 for unit %s"]
    pub conf03: CONF0,
    #[doc = "0x28 - Configuration register 1 for unit %s"]
    pub conf13: CONF1,
    #[doc = "0x2c - Configuration register 2 for unit %s"]
    pub conf23: CONF2,
    #[doc = "0x30..0x40 - Counter value for unit %s"]
    pub cnt: [CNT; 4],
    #[doc = "0x40 - Interrupt raw status register"]
    pub int_raw: INT_RAW,
    #[doc = "0x44 - Interrupt status register"]
    pub int_st: INT_ST,
    #[doc = "0x48 - Interrupt enable register"]
    pub int_ena: INT_ENA,
    #[doc = "0x4c - Interrupt clear register"]
    pub int_clr: INT_CLR,
    #[doc = "0x50..0x60 - PNCT UNIT%s status register"]
    pub status: [STATUS; 4],
    #[doc = "0x60 - Control register for all counters"]
    pub ctrl: CTRL,
    _reserved19: [u8; 0x98],
    #[doc = "0xfc - PCNT version control register"]
    pub date: DATE,
}
#[doc = "CONF0 (rw) register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "Configuration register 0 for unit %s"]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "Configuration register 1 for unit %s"]
pub mod conf1;
#[doc = "CONF2 (rw) register accessor: an alias for `Reg<CONF2_SPEC>`"]
pub type CONF2 = crate::Reg<conf2::CONF2_SPEC>;
#[doc = "Configuration register 2 for unit %s"]
pub mod conf2;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter value for unit %s"]
pub mod cnt;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw status register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt status register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod int_clr;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "PNCT UNIT%s status register"]
pub mod status;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register for all counters"]
pub mod ctrl;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "PCNT version control register"]
pub mod date;
