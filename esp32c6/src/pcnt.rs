#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration register 0 for unit %s"]
    pub u0_conf0: U_CONF0,
    #[doc = "0x04 - Configuration register 1 for unit %s"]
    pub u0_conf1: U_CONF1,
    #[doc = "0x08 - Configuration register 2 for unit %s"]
    pub u0_conf2: U_CONF2,
    #[doc = "0x0c - Configuration register 0 for unit %s"]
    pub u1_conf0: U_CONF0,
    #[doc = "0x10 - Configuration register 1 for unit %s"]
    pub u1_conf1: U_CONF1,
    #[doc = "0x14 - Configuration register 2 for unit %s"]
    pub u1_conf2: U_CONF2,
    #[doc = "0x18 - Configuration register 0 for unit %s"]
    pub u2_conf0: U_CONF0,
    #[doc = "0x1c - Configuration register 1 for unit %s"]
    pub u2_conf1: U_CONF1,
    #[doc = "0x20 - Configuration register 2 for unit %s"]
    pub u2_conf2: U_CONF2,
    #[doc = "0x24 - Configuration register 0 for unit %s"]
    pub u3_conf0: U_CONF0,
    #[doc = "0x28 - Configuration register 1 for unit %s"]
    pub u3_conf1: U_CONF1,
    #[doc = "0x2c - Configuration register 2 for unit %s"]
    pub u3_conf2: U_CONF2,
    #[doc = "0x30..0x40 - Counter value for unit %s"]
    pub u_cnt: [U_CNT; 4],
    #[doc = "0x40 - Interrupt raw status register"]
    pub int_raw: INT_RAW,
    #[doc = "0x44 - Interrupt status register"]
    pub int_st: INT_ST,
    #[doc = "0x48 - Interrupt enable register"]
    pub int_ena: INT_ENA,
    #[doc = "0x4c - Interrupt clear register"]
    pub int_clr: INT_CLR,
    #[doc = "0x50..0x60 - PNCT UNIT%s status register"]
    pub u_status: [U_STATUS; 4],
    #[doc = "0x60 - Control register for all counters"]
    pub ctrl: CTRL,
    _reserved19: [u8; 0x98],
    #[doc = "0xfc - PCNT version control register"]
    pub date: DATE,
}
#[doc = "U_CONF0 (rw) register accessor: an alias for `Reg<U_CONF0_SPEC>`"]
pub type U_CONF0 = crate::Reg<u_conf0::U_CONF0_SPEC>;
#[doc = "Configuration register 0 for unit %s"]
pub mod u_conf0;
#[doc = "U_CONF1 (rw) register accessor: an alias for `Reg<U_CONF1_SPEC>`"]
pub type U_CONF1 = crate::Reg<u_conf1::U_CONF1_SPEC>;
#[doc = "Configuration register 1 for unit %s"]
pub mod u_conf1;
#[doc = "U_CONF2 (rw) register accessor: an alias for `Reg<U_CONF2_SPEC>`"]
pub type U_CONF2 = crate::Reg<u_conf2::U_CONF2_SPEC>;
#[doc = "Configuration register 2 for unit %s"]
pub mod u_conf2;
#[doc = "U_CNT (r) register accessor: an alias for `Reg<U_CNT_SPEC>`"]
pub type U_CNT = crate::Reg<u_cnt::U_CNT_SPEC>;
#[doc = "Counter value for unit %s"]
pub mod u_cnt;
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
#[doc = "U_STATUS (r) register accessor: an alias for `Reg<U_STATUS_SPEC>`"]
pub type U_STATUS = crate::Reg<u_status::U_STATUS_SPEC>;
#[doc = "PNCT UNIT%s status register"]
pub mod u_status;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register for all counters"]
pub mod ctrl;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "PCNT version control register"]
pub mod date;
