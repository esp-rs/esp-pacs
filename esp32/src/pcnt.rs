#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub conf00: CONF0,
    #[doc = "0x04 - "]
    pub conf10: CONF1,
    #[doc = "0x08 - "]
    pub conf20: CONF2,
    #[doc = "0x0c - "]
    pub conf01: CONF0,
    #[doc = "0x10 - "]
    pub conf11: CONF1,
    #[doc = "0x14 - "]
    pub conf21: CONF2,
    #[doc = "0x18 - "]
    pub conf02: CONF0,
    #[doc = "0x1c - "]
    pub conf12: CONF1,
    #[doc = "0x20 - "]
    pub conf22: CONF2,
    #[doc = "0x24 - "]
    pub conf03: CONF0,
    #[doc = "0x28 - "]
    pub conf13: CONF1,
    #[doc = "0x2c - "]
    pub conf23: CONF2,
    #[doc = "0x30 - "]
    pub conf04: CONF0,
    #[doc = "0x34 - "]
    pub conf14: CONF1,
    #[doc = "0x38 - "]
    pub conf24: CONF2,
    #[doc = "0x3c - "]
    pub conf05: CONF0,
    #[doc = "0x40 - "]
    pub conf15: CONF1,
    #[doc = "0x44 - "]
    pub conf25: CONF2,
    #[doc = "0x48 - "]
    pub conf06: CONF0,
    #[doc = "0x4c - "]
    pub conf16: CONF1,
    #[doc = "0x50 - "]
    pub conf26: CONF2,
    #[doc = "0x54 - "]
    pub conf07: CONF0,
    #[doc = "0x58 - "]
    pub conf17: CONF1,
    #[doc = "0x5c - "]
    pub conf27: CONF2,
    #[doc = "0x60..0x80 - "]
    pub cnt: [CNT; 8],
    #[doc = "0x80 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x84 - "]
    pub int_st: INT_ST,
    #[doc = "0x88 - "]
    pub int_ena: INT_ENA,
    #[doc = "0x8c - "]
    pub int_clr: INT_CLR,
    #[doc = "0x90..0xb0 - "]
    pub status: [STATUS; 8],
    #[doc = "0xb0 - "]
    pub ctrl: CTRL,
    _reserved31: [u8; 0x48],
    #[doc = "0xfc - "]
    pub date: DATE,
}
#[doc = "CONF0 (rw) register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = ""]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = ""]
pub mod conf1;
#[doc = "CONF2 (rw) register accessor: an alias for `Reg<CONF2_SPEC>`"]
pub type CONF2 = crate::Reg<conf2::CONF2_SPEC>;
#[doc = ""]
pub mod conf2;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = ""]
pub mod cnt;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
