#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Dedicated GPIO directive output register"]
    pub out_drt: OUT_DRT,
    #[doc = "0x04 - Dedicated GPIO mask output register"]
    pub out_msk: OUT_MSK,
    #[doc = "0x08 - Dedicated GPIO individual output register"]
    pub out_idv: OUT_IDV,
    #[doc = "0x0c - Dedicated GPIO output status register"]
    pub out_scan: OUT_SCAN,
    #[doc = "0x10 - Dedicated GPIO output mode selection register"]
    pub out_cpu: OUT_CPU,
    #[doc = "0x14 - Dedicated GPIO input delay configuration register"]
    pub in_dly: IN_DLY,
    #[doc = "0x18 - Dedicated GPIO input status register"]
    pub in_scan: IN_SCAN,
    #[doc = "0x1c - Dedicated GPIO interrupts generation mode register"]
    pub intr_rcgn: INTR_RCGN,
    #[doc = "0x20 - Raw interrupt status"]
    pub intr_raw: INTR_RAW,
    #[doc = "0x24 - Interrupt enable bits"]
    pub intr_rls: INTR_RLS,
    #[doc = "0x28 - Masked interrupt status"]
    pub intr_st: INTR_ST,
    #[doc = "0x2c - Interrupt clear bits"]
    pub intr_clr: INTR_CLR,
}
#[doc = "OUT_DRT (w) register accessor: an alias for `Reg<OUT_DRT_SPEC>`"]
pub type OUT_DRT = crate::Reg<out_drt::OUT_DRT_SPEC>;
#[doc = "Dedicated GPIO directive output register"]
pub mod out_drt;
#[doc = "OUT_MSK (w) register accessor: an alias for `Reg<OUT_MSK_SPEC>`"]
pub type OUT_MSK = crate::Reg<out_msk::OUT_MSK_SPEC>;
#[doc = "Dedicated GPIO mask output register"]
pub mod out_msk;
#[doc = "OUT_IDV (w) register accessor: an alias for `Reg<OUT_IDV_SPEC>`"]
pub type OUT_IDV = crate::Reg<out_idv::OUT_IDV_SPEC>;
#[doc = "Dedicated GPIO individual output register"]
pub mod out_idv;
#[doc = "OUT_SCAN (r) register accessor: an alias for `Reg<OUT_SCAN_SPEC>`"]
pub type OUT_SCAN = crate::Reg<out_scan::OUT_SCAN_SPEC>;
#[doc = "Dedicated GPIO output status register"]
pub mod out_scan;
#[doc = "OUT_CPU (rw) register accessor: an alias for `Reg<OUT_CPU_SPEC>`"]
pub type OUT_CPU = crate::Reg<out_cpu::OUT_CPU_SPEC>;
#[doc = "Dedicated GPIO output mode selection register"]
pub mod out_cpu;
#[doc = "IN_DLY (rw) register accessor: an alias for `Reg<IN_DLY_SPEC>`"]
pub type IN_DLY = crate::Reg<in_dly::IN_DLY_SPEC>;
#[doc = "Dedicated GPIO input delay configuration register"]
pub mod in_dly;
#[doc = "IN_SCAN (r) register accessor: an alias for `Reg<IN_SCAN_SPEC>`"]
pub type IN_SCAN = crate::Reg<in_scan::IN_SCAN_SPEC>;
#[doc = "Dedicated GPIO input status register"]
pub mod in_scan;
#[doc = "INTR_RCGN (rw) register accessor: an alias for `Reg<INTR_RCGN_SPEC>`"]
pub type INTR_RCGN = crate::Reg<intr_rcgn::INTR_RCGN_SPEC>;
#[doc = "Dedicated GPIO interrupts generation mode register"]
pub mod intr_rcgn;
#[doc = "INTR_RAW (r) register accessor: an alias for `Reg<INTR_RAW_SPEC>`"]
pub type INTR_RAW = crate::Reg<intr_raw::INTR_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod intr_raw;
#[doc = "INTR_RLS (rw) register accessor: an alias for `Reg<INTR_RLS_SPEC>`"]
pub type INTR_RLS = crate::Reg<intr_rls::INTR_RLS_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod intr_rls;
#[doc = "INTR_ST (r) register accessor: an alias for `Reg<INTR_ST_SPEC>`"]
pub type INTR_ST = crate::Reg<intr_st::INTR_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod intr_st;
#[doc = "INTR_CLR (w) register accessor: an alias for `Reg<INTR_CLR_SPEC>`"]
pub type INTR_CLR = crate::Reg<intr_clr::INTR_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod intr_clr;
