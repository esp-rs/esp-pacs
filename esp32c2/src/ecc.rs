#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - I2S interrupt raw register, valid in level."]
    pub mult_int_raw: MULT_INT_RAW,
    #[doc = "0x10 - I2S interrupt status register."]
    pub mult_int_st: MULT_INT_ST,
    #[doc = "0x14 - I2S interrupt enable register."]
    pub mult_int_ena: MULT_INT_ENA,
    #[doc = "0x18 - I2S interrupt clear register."]
    pub mult_int_clr: MULT_INT_CLR,
    #[doc = "0x1c - I2S RX configure register"]
    pub mult_conf: MULT_CONF,
    _reserved5: [u8; 0xdc],
    #[doc = "0xfc - Version control register"]
    pub mult_date: MULT_DATE,
}
#[doc = "MULT_INT_RAW (r) register accessor: an alias for `Reg<MULT_INT_RAW_SPEC>`"]
pub type MULT_INT_RAW = crate::Reg<mult_int_raw::MULT_INT_RAW_SPEC>;
#[doc = "I2S interrupt raw register, valid in level."]
pub mod mult_int_raw;
#[doc = "MULT_INT_ST (r) register accessor: an alias for `Reg<MULT_INT_ST_SPEC>`"]
pub type MULT_INT_ST = crate::Reg<mult_int_st::MULT_INT_ST_SPEC>;
#[doc = "I2S interrupt status register."]
pub mod mult_int_st;
#[doc = "MULT_INT_ENA (rw) register accessor: an alias for `Reg<MULT_INT_ENA_SPEC>`"]
pub type MULT_INT_ENA = crate::Reg<mult_int_ena::MULT_INT_ENA_SPEC>;
#[doc = "I2S interrupt enable register."]
pub mod mult_int_ena;
#[doc = "MULT_INT_CLR (w) register accessor: an alias for `Reg<MULT_INT_CLR_SPEC>`"]
pub type MULT_INT_CLR = crate::Reg<mult_int_clr::MULT_INT_CLR_SPEC>;
#[doc = "I2S interrupt clear register."]
pub mod mult_int_clr;
#[doc = "MULT_CONF (rw) register accessor: an alias for `Reg<MULT_CONF_SPEC>`"]
pub type MULT_CONF = crate::Reg<mult_conf::MULT_CONF_SPEC>;
#[doc = "I2S RX configure register"]
pub mod mult_conf;
#[doc = "MULT_DATE (rw) register accessor: an alias for `Reg<MULT_DATE_SPEC>`"]
pub type MULT_DATE = crate::Reg<mult_date::MULT_DATE_SPEC>;
#[doc = "Version control register"]
pub mod mult_date;
