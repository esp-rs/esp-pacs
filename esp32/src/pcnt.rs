#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub u0_conf0: U_CONF0,
    #[doc = "0x04 - "]
    pub u0_conf1: U_CONF1,
    #[doc = "0x08 - "]
    pub u0_conf2: U_CONF2,
    #[doc = "0x0c - "]
    pub u1_conf0: U_CONF0,
    #[doc = "0x10 - "]
    pub u1_conf1: U_CONF1,
    #[doc = "0x14 - "]
    pub u1_conf2: U_CONF2,
    #[doc = "0x18 - "]
    pub u2_conf0: U_CONF0,
    #[doc = "0x1c - "]
    pub u2_conf1: U_CONF1,
    #[doc = "0x20 - "]
    pub u2_conf2: U_CONF2,
    #[doc = "0x24 - "]
    pub u3_conf0: U_CONF0,
    #[doc = "0x28 - "]
    pub u3_conf1: U_CONF1,
    #[doc = "0x2c - "]
    pub u3_conf2: U_CONF2,
    #[doc = "0x30 - "]
    pub u4_conf0: U_CONF0,
    #[doc = "0x34 - "]
    pub u4_conf1: U_CONF1,
    #[doc = "0x38 - "]
    pub u4_conf2: U_CONF2,
    #[doc = "0x3c - "]
    pub u5_conf0: U_CONF0,
    #[doc = "0x40 - "]
    pub u5_conf1: U_CONF1,
    #[doc = "0x44 - "]
    pub u5_conf2: U_CONF2,
    #[doc = "0x48 - "]
    pub u6_conf0: U_CONF0,
    #[doc = "0x4c - "]
    pub u6_conf1: U_CONF1,
    #[doc = "0x50 - "]
    pub u6_conf2: U_CONF2,
    #[doc = "0x54 - "]
    pub u7_conf0: U_CONF0,
    #[doc = "0x58 - "]
    pub u7_conf1: U_CONF1,
    #[doc = "0x5c - "]
    pub u7_conf2: U_CONF2,
    #[doc = "0x60..0x80 - "]
    pub u_cnt: [U_CNT; 8],
    #[doc = "0x80 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x84 - "]
    pub int_st: INT_ST,
    #[doc = "0x88 - "]
    pub int_ena: INT_ENA,
    #[doc = "0x8c - "]
    pub int_clr: INT_CLR,
    #[doc = "0x90..0xb0 - "]
    pub u_status: [U_STATUS; 8],
    #[doc = "0xb0 - "]
    pub ctrl: CTRL,
    _reserved31: [u8; 0x48],
    #[doc = "0xfc - "]
    pub date: DATE,
}
#[doc = "U_CONF0 (rw) register accessor: an alias for `Reg<U_CONF0_SPEC>`"]
pub type U_CONF0 = crate::Reg<u_conf0::U_CONF0_SPEC>;
#[doc = ""]
pub mod u_conf0;
#[doc = "U_CONF1 (rw) register accessor: an alias for `Reg<U_CONF1_SPEC>`"]
pub type U_CONF1 = crate::Reg<u_conf1::U_CONF1_SPEC>;
#[doc = ""]
pub mod u_conf1;
#[doc = "U_CONF2 (rw) register accessor: an alias for `Reg<U_CONF2_SPEC>`"]
pub type U_CONF2 = crate::Reg<u_conf2::U_CONF2_SPEC>;
#[doc = ""]
pub mod u_conf2;
#[doc = "U_CNT (r) register accessor: an alias for `Reg<U_CNT_SPEC>`"]
pub type U_CNT = crate::Reg<u_cnt::U_CNT_SPEC>;
#[doc = ""]
pub mod u_cnt;
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
#[doc = "U_STATUS (rw) register accessor: an alias for `Reg<U_STATUS_SPEC>`"]
pub type U_STATUS = crate::Reg<u_status::U_STATUS_SPEC>;
#[doc = ""]
pub mod u_status;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
