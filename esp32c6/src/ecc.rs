#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - ECC interrupt raw register, valid in level."]
    pub mult_int_raw: MULT_INT_RAW,
    #[doc = "0x10 - ECC interrupt status register."]
    pub mult_int_st: MULT_INT_ST,
    #[doc = "0x14 - ECC interrupt enable register."]
    pub mult_int_ena: MULT_INT_ENA,
    #[doc = "0x18 - ECC interrupt clear register."]
    pub mult_int_clr: MULT_INT_CLR,
    #[doc = "0x1c - ECC configure register"]
    pub mult_conf: MULT_CONF,
    _reserved5: [u8; 0xdc],
    #[doc = "0xfc - Version control register"]
    pub mult_date: MULT_DATE,
    #[doc = "0x100..0x120 - The memory that stores k."]
    pub k_mem: [K_MEM; 32],
    #[doc = "0x120..0x140 - The memory that stores Px."]
    pub px_mem: [PX_MEM; 32],
    #[doc = "0x140..0x160 - The memory that stores Py."]
    pub py_mem: [PY_MEM; 32],
}
#[doc = "MULT_INT_RAW (r) register accessor: an alias for `Reg<MULT_INT_RAW_SPEC>`"]
pub type MULT_INT_RAW = crate::Reg<mult_int_raw::MULT_INT_RAW_SPEC>;
#[doc = "ECC interrupt raw register, valid in level."]
pub mod mult_int_raw;
#[doc = "MULT_INT_ST (r) register accessor: an alias for `Reg<MULT_INT_ST_SPEC>`"]
pub type MULT_INT_ST = crate::Reg<mult_int_st::MULT_INT_ST_SPEC>;
#[doc = "ECC interrupt status register."]
pub mod mult_int_st;
#[doc = "MULT_INT_ENA (rw) register accessor: an alias for `Reg<MULT_INT_ENA_SPEC>`"]
pub type MULT_INT_ENA = crate::Reg<mult_int_ena::MULT_INT_ENA_SPEC>;
#[doc = "ECC interrupt enable register."]
pub mod mult_int_ena;
#[doc = "MULT_INT_CLR (w) register accessor: an alias for `Reg<MULT_INT_CLR_SPEC>`"]
pub type MULT_INT_CLR = crate::Reg<mult_int_clr::MULT_INT_CLR_SPEC>;
#[doc = "ECC interrupt clear register."]
pub mod mult_int_clr;
#[doc = "MULT_CONF (rw) register accessor: an alias for `Reg<MULT_CONF_SPEC>`"]
pub type MULT_CONF = crate::Reg<mult_conf::MULT_CONF_SPEC>;
#[doc = "ECC configure register"]
pub mod mult_conf;
#[doc = "MULT_DATE (rw) register accessor: an alias for `Reg<MULT_DATE_SPEC>`"]
pub type MULT_DATE = crate::Reg<mult_date::MULT_DATE_SPEC>;
#[doc = "Version control register"]
pub mod mult_date;
#[doc = "K_MEM (rw) register accessor: an alias for `Reg<K_MEM_SPEC>`"]
pub type K_MEM = crate::Reg<k_mem::K_MEM_SPEC>;
#[doc = "The memory that stores k."]
pub mod k_mem;
#[doc = "PX_MEM (rw) register accessor: an alias for `Reg<PX_MEM_SPEC>`"]
pub type PX_MEM = crate::Reg<px_mem::PX_MEM_SPEC>;
#[doc = "The memory that stores Px."]
pub mod px_mem;
#[doc = "PY_MEM (rw) register accessor: an alias for `Reg<PY_MEM_SPEC>`"]
pub type PY_MEM = crate::Reg<py_mem::PY_MEM_SPEC>;
#[doc = "The memory that stores Py."]
pub mod py_mem;
