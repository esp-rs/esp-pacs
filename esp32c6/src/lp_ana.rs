#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub bod_mode0_cntl: BOD_MODE0_CNTL,
    #[doc = "0x04 - need_des"]
    pub bod_mode1_cntl: BOD_MODE1_CNTL,
    #[doc = "0x08 - need_des"]
    pub ck_glitch_cntl: CK_GLITCH_CNTL,
    #[doc = "0x0c - need_des"]
    pub fib_enable: FIB_ENABLE,
    #[doc = "0x10 - need_des"]
    pub int_raw: INT_RAW,
    #[doc = "0x14 - need_des"]
    pub int_st: INT_ST,
    #[doc = "0x18 - need_des"]
    pub int_ena: INT_ENA,
    #[doc = "0x1c - need_des"]
    pub int_clr: INT_CLR,
    #[doc = "0x20 - need_des"]
    pub lp_int_raw: LP_INT_RAW,
    #[doc = "0x24 - need_des"]
    pub lp_int_st: LP_INT_ST,
    #[doc = "0x28 - need_des"]
    pub lp_int_ena: LP_INT_ENA,
    #[doc = "0x2c - need_des"]
    pub lp_int_clr: LP_INT_CLR,
    _reserved12: [u8; 0x03cc],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "BOD_MODE0_CNTL (rw) register accessor: an alias for `Reg<BOD_MODE0_CNTL_SPEC>`"]
pub type BOD_MODE0_CNTL = crate::Reg<bod_mode0_cntl::BOD_MODE0_CNTL_SPEC>;
#[doc = "need_des"]
pub mod bod_mode0_cntl;
#[doc = "BOD_MODE1_CNTL (rw) register accessor: an alias for `Reg<BOD_MODE1_CNTL_SPEC>`"]
pub type BOD_MODE1_CNTL = crate::Reg<bod_mode1_cntl::BOD_MODE1_CNTL_SPEC>;
#[doc = "need_des"]
pub mod bod_mode1_cntl;
#[doc = "CK_GLITCH_CNTL (rw) register accessor: an alias for `Reg<CK_GLITCH_CNTL_SPEC>`"]
pub type CK_GLITCH_CNTL = crate::Reg<ck_glitch_cntl::CK_GLITCH_CNTL_SPEC>;
#[doc = "need_des"]
pub mod ck_glitch_cntl;
#[doc = "FIB_ENABLE (rw) register accessor: an alias for `Reg<FIB_ENABLE_SPEC>`"]
pub type FIB_ENABLE = crate::Reg<fib_enable::FIB_ENABLE_SPEC>;
#[doc = "need_des"]
pub mod fib_enable;
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
#[doc = "LP_INT_RAW (rw) register accessor: an alias for `Reg<LP_INT_RAW_SPEC>`"]
pub type LP_INT_RAW = crate::Reg<lp_int_raw::LP_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod lp_int_raw;
#[doc = "LP_INT_ST (r) register accessor: an alias for `Reg<LP_INT_ST_SPEC>`"]
pub type LP_INT_ST = crate::Reg<lp_int_st::LP_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod lp_int_st;
#[doc = "LP_INT_ENA (rw) register accessor: an alias for `Reg<LP_INT_ENA_SPEC>`"]
pub type LP_INT_ENA = crate::Reg<lp_int_ena::LP_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod lp_int_ena;
#[doc = "LP_INT_CLR (w) register accessor: an alias for `Reg<LP_INT_CLR_SPEC>`"]
pub type LP_INT_CLR = crate::Reg<lp_int_clr::LP_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod lp_int_clr;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
