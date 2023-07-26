#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - "]
    pub chdata: [CHDATA; 8],
    #[doc = "0x20 - "]
    pub ch0conf0: CHCONF0,
    #[doc = "0x24 - "]
    pub ch0conf1: CHCONF1,
    #[doc = "0x28 - "]
    pub ch1conf0: CHCONF0,
    #[doc = "0x2c - "]
    pub ch1conf1: CHCONF1,
    #[doc = "0x30 - "]
    pub ch2conf0: CHCONF0,
    #[doc = "0x34 - "]
    pub ch2conf1: CHCONF1,
    #[doc = "0x38 - "]
    pub ch3conf0: CHCONF0,
    #[doc = "0x3c - "]
    pub ch3conf1: CHCONF1,
    #[doc = "0x40 - "]
    pub ch4conf0: CHCONF0,
    #[doc = "0x44 - "]
    pub ch4conf1: CHCONF1,
    #[doc = "0x48 - "]
    pub ch5conf0: CHCONF0,
    #[doc = "0x4c - "]
    pub ch5conf1: CHCONF1,
    #[doc = "0x50 - "]
    pub ch6conf0: CHCONF0,
    #[doc = "0x54 - "]
    pub ch6conf1: CHCONF1,
    #[doc = "0x58 - "]
    pub ch7conf0: CHCONF0,
    #[doc = "0x5c - "]
    pub ch7conf1: CHCONF1,
    #[doc = "0x60..0x80 - "]
    pub chstatus: [CHSTATUS; 8],
    #[doc = "0x80..0xa0 - "]
    pub chaddr: [CHADDR; 8],
    #[doc = "0xa0 - "]
    pub int_raw: INT_RAW,
    #[doc = "0xa4 - "]
    pub int_st: INT_ST,
    #[doc = "0xa8 - "]
    pub int_ena: INT_ENA,
    #[doc = "0xac - "]
    pub int_clr: INT_CLR,
    #[doc = "0xb0..0xd0 - "]
    pub chcarrier_duty: [CHCARRIER_DUTY; 8],
    #[doc = "0xd0..0xf0 - "]
    pub ch_tx_lim: [CH_TX_LIM; 8],
    #[doc = "0xf0 - "]
    pub apb_conf: APB_CONF,
    _reserved26: [u8; 0x08],
    #[doc = "0xfc - "]
    pub date: DATE,
}
#[doc = "CHDATA (rw) register accessor: an alias for `Reg<CHDATA_SPEC>`"]
pub type CHDATA = crate::Reg<chdata::CHDATA_SPEC>;
#[doc = ""]
pub mod chdata;
#[doc = "CHCONF0 (rw) register accessor: an alias for `Reg<CHCONF0_SPEC>`"]
pub type CHCONF0 = crate::Reg<chconf0::CHCONF0_SPEC>;
#[doc = ""]
pub mod chconf0;
#[doc = "CHCONF1 (rw) register accessor: an alias for `Reg<CHCONF1_SPEC>`"]
pub type CHCONF1 = crate::Reg<chconf1::CHCONF1_SPEC>;
#[doc = ""]
pub mod chconf1;
#[doc = "CHSTATUS (r) register accessor: an alias for `Reg<CHSTATUS_SPEC>`"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = ""]
pub mod chstatus;
#[doc = "CHADDR (r) register accessor: an alias for `Reg<CHADDR_SPEC>`"]
pub type CHADDR = crate::Reg<chaddr::CHADDR_SPEC>;
#[doc = ""]
pub mod chaddr;
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
#[doc = "CHCARRIER_DUTY (rw) register accessor: an alias for `Reg<CHCARRIER_DUTY_SPEC>`"]
pub type CHCARRIER_DUTY = crate::Reg<chcarrier_duty::CHCARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod chcarrier_duty;
#[doc = "CH_TX_LIM (rw) register accessor: an alias for `Reg<CH_TX_LIM_SPEC>`"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = ""]
pub mod ch_tx_lim;
#[doc = "APB_CONF (rw) register accessor: an alias for `Reg<APB_CONF_SPEC>`"]
pub type APB_CONF = crate::Reg<apb_conf::APB_CONF_SPEC>;
#[doc = ""]
pub mod apb_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
