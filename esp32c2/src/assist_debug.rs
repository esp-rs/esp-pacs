#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - core0 monitor enable configuration register"]
    pub core_0_montr_ena: CORE_0_MONTR_ENA,
    #[doc = "0x04 - core0 monitor interrupt status register"]
    pub core_0_intr_raw: CORE_0_INTR_RAW,
    #[doc = "0x08 - core0 monitor interrupt enable register"]
    pub core_0_intr_ena: CORE_0_INTR_ENA,
    #[doc = "0x0c - core0 monitor interrupt clr register"]
    pub core_0_intr_clr: CORE_0_INTR_CLR,
    #[doc = "0x10 - stack min value"]
    pub core_0_sp_min: CORE_0_SP_MIN,
    #[doc = "0x14 - stack max value"]
    pub core_0_sp_max: CORE_0_SP_MAX,
    #[doc = "0x18 - stack monitor pc status register"]
    pub core_0_sp_pc: CORE_0_SP_PC,
    #[doc = "0x1c - record enable configuration register"]
    pub core_0_rcd_en: CORE_0_RCD_EN,
    #[doc = "0x20 - record status regsiter"]
    pub core_0_rcd_pdebugpc: CORE_0_RCD_PDEBUGPC,
    #[doc = "0x24 - record status regsiter"]
    pub core_0_rcd_pdebugsp: CORE_0_RCD_PDEBUGSP,
    #[doc = "0x28 - cpu status register"]
    pub core_0_lastpc_before_exception: CORE_0_LASTPC_BEFORE_EXCEPTION,
    #[doc = "0x2c - cpu status register"]
    pub core_0_debug_mode: CORE_0_DEBUG_MODE,
    #[doc = "0x30 - clock gate register"]
    pub clock_gate: CLOCK_GATE,
    _reserved13: [u8; 0x01c8],
    #[doc = "0x1fc - version register"]
    pub date: DATE,
}
#[doc = "CORE_0_MONTR_ENA (rw) register accessor: an alias for `Reg<CORE_0_MONTR_ENA_SPEC>`"]
pub type CORE_0_MONTR_ENA = crate::Reg<core_0_montr_ena::CORE_0_MONTR_ENA_SPEC>;
#[doc = "core0 monitor enable configuration register"]
pub mod core_0_montr_ena;
#[doc = "CORE_0_INTR_RAW (r) register accessor: an alias for `Reg<CORE_0_INTR_RAW_SPEC>`"]
pub type CORE_0_INTR_RAW = crate::Reg<core_0_intr_raw::CORE_0_INTR_RAW_SPEC>;
#[doc = "core0 monitor interrupt status register"]
pub mod core_0_intr_raw;
#[doc = "CORE_0_INTR_ENA (rw) register accessor: an alias for `Reg<CORE_0_INTR_ENA_SPEC>`"]
pub type CORE_0_INTR_ENA = crate::Reg<core_0_intr_ena::CORE_0_INTR_ENA_SPEC>;
#[doc = "core0 monitor interrupt enable register"]
pub mod core_0_intr_ena;
#[doc = "CORE_0_INTR_CLR (w) register accessor: an alias for `Reg<CORE_0_INTR_CLR_SPEC>`"]
pub type CORE_0_INTR_CLR = crate::Reg<core_0_intr_clr::CORE_0_INTR_CLR_SPEC>;
#[doc = "core0 monitor interrupt clr register"]
pub mod core_0_intr_clr;
#[doc = "CORE_0_SP_MIN (rw) register accessor: an alias for `Reg<CORE_0_SP_MIN_SPEC>`"]
pub type CORE_0_SP_MIN = crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>;
#[doc = "stack min value"]
pub mod core_0_sp_min;
#[doc = "CORE_0_SP_MAX (rw) register accessor: an alias for `Reg<CORE_0_SP_MAX_SPEC>`"]
pub type CORE_0_SP_MAX = crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>;
#[doc = "stack max value"]
pub mod core_0_sp_max;
#[doc = "CORE_0_SP_PC (r) register accessor: an alias for `Reg<CORE_0_SP_PC_SPEC>`"]
pub type CORE_0_SP_PC = crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>;
#[doc = "stack monitor pc status register"]
pub mod core_0_sp_pc;
#[doc = "CORE_0_RCD_EN (rw) register accessor: an alias for `Reg<CORE_0_RCD_EN_SPEC>`"]
pub type CORE_0_RCD_EN = crate::Reg<core_0_rcd_en::CORE_0_RCD_EN_SPEC>;
#[doc = "record enable configuration register"]
pub mod core_0_rcd_en;
#[doc = "CORE_0_RCD_PDEBUGPC (r) register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGPC_SPEC>`"]
pub type CORE_0_RCD_PDEBUGPC = crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>;
#[doc = "record status regsiter"]
pub mod core_0_rcd_pdebugpc;
#[doc = "CORE_0_RCD_PDEBUGSP (r) register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGSP_SPEC>`"]
pub type CORE_0_RCD_PDEBUGSP = crate::Reg<core_0_rcd_pdebugsp::CORE_0_RCD_PDEBUGSP_SPEC>;
#[doc = "record status regsiter"]
pub mod core_0_rcd_pdebugsp;
#[doc = "CORE_0_LASTPC_BEFORE_EXCEPTION (r) register accessor: an alias for `Reg<CORE_0_LASTPC_BEFORE_EXCEPTION_SPEC>`"]
pub type CORE_0_LASTPC_BEFORE_EXCEPTION =
    crate::Reg<core_0_lastpc_before_exception::CORE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "cpu status register"]
pub mod core_0_lastpc_before_exception;
#[doc = "CORE_0_DEBUG_MODE (r) register accessor: an alias for `Reg<CORE_0_DEBUG_MODE_SPEC>`"]
pub type CORE_0_DEBUG_MODE = crate::Reg<core_0_debug_mode::CORE_0_DEBUG_MODE_SPEC>;
#[doc = "cpu status register"]
pub mod core_0_debug_mode;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
