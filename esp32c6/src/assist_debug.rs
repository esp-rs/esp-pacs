#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - core0 monitor enable configuration register"]
    pub core_0_montr_ena: CORE_0_MONTR_ENA,
    #[doc = "0x04 - core0 monitor interrupt status register"]
    pub core_0_intr_raw: CORE_0_INTR_RAW,
    #[doc = "0x08 - core0 monitor interrupt enable register"]
    pub core_0_intr_ena: CORE_0_INTR_ENA,
    #[doc = "0x0c - core0 monitor interrupt clr register"]
    pub core_0_intr_clr: CORE_0_INTR_CLR,
    #[doc = "0x10 - core0 dram0 region0 addr configuration register"]
    pub core_0_area_dram0_0_min: CORE_0_AREA_DRAM0_0_MIN,
    #[doc = "0x14 - core0 dram0 region0 addr configuration register"]
    pub core_0_area_dram0_0_max: CORE_0_AREA_DRAM0_0_MAX,
    #[doc = "0x18 - core0 dram0 region1 addr configuration register"]
    pub core_0_area_dram0_1_min: CORE_0_AREA_DRAM0_1_MIN,
    #[doc = "0x1c - core0 dram0 region1 addr configuration register"]
    pub core_0_area_dram0_1_max: CORE_0_AREA_DRAM0_1_MAX,
    #[doc = "0x20 - core0 PIF region0 addr configuration register"]
    pub core_0_area_pif_0_min: CORE_0_AREA_PIF_0_MIN,
    #[doc = "0x24 - core0 PIF region0 addr configuration register"]
    pub core_0_area_pif_0_max: CORE_0_AREA_PIF_0_MAX,
    #[doc = "0x28 - core0 PIF region1 addr configuration register"]
    pub core_0_area_pif_1_min: CORE_0_AREA_PIF_1_MIN,
    #[doc = "0x2c - core0 PIF region1 addr configuration register"]
    pub core_0_area_pif_1_max: CORE_0_AREA_PIF_1_MAX,
    #[doc = "0x30 - core0 area pc status register"]
    pub core_0_area_pc: CORE_0_AREA_PC,
    #[doc = "0x34 - core0 area sp status register"]
    pub core_0_area_sp: CORE_0_AREA_SP,
    #[doc = "0x38 - stack min value"]
    pub core_0_sp_min: CORE_0_SP_MIN,
    #[doc = "0x3c - stack max value"]
    pub core_0_sp_max: CORE_0_SP_MAX,
    #[doc = "0x40 - stack monitor pc status register"]
    pub core_0_sp_pc: CORE_0_SP_PC,
    #[doc = "0x44 - record enable configuration register"]
    pub core_0_rcd_en: CORE_0_RCD_EN,
    #[doc = "0x48 - record status regsiter"]
    pub core_0_rcd_pdebugpc: CORE_0_RCD_PDEBUGPC,
    #[doc = "0x4c - record status regsiter"]
    pub core_0_rcd_pdebugsp: CORE_0_RCD_PDEBUGSP,
    #[doc = "0x50 - exception monitor status register0"]
    pub core_0_iram0_exception_monitor_0: CORE_0_IRAM0_EXCEPTION_MONITOR_0,
    #[doc = "0x54 - exception monitor status register1"]
    pub core_0_iram0_exception_monitor_1: CORE_0_IRAM0_EXCEPTION_MONITOR_1,
    #[doc = "0x58 - exception monitor status register2"]
    pub core_0_dram0_exception_monitor_0: CORE_0_DRAM0_EXCEPTION_MONITOR_0,
    #[doc = "0x5c - exception monitor status register3"]
    pub core_0_dram0_exception_monitor_1: CORE_0_DRAM0_EXCEPTION_MONITOR_1,
    #[doc = "0x60 - exception monitor status register4"]
    pub core_0_dram0_exception_monitor_2: CORE_0_DRAM0_EXCEPTION_MONITOR_2,
    #[doc = "0x64 - exception monitor status register5"]
    pub core_0_dram0_exception_monitor_3: CORE_0_DRAM0_EXCEPTION_MONITOR_3,
    #[doc = "0x68 - exception monitor status register6"]
    pub core_x_iram0_dram0_exception_monitor_0: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0,
    #[doc = "0x6c - exception monitor status register7"]
    pub core_x_iram0_dram0_exception_monitor_1: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1,
    #[doc = "0x70 - cpu status register"]
    pub c0re_0_lastpc_before_exception: C0RE_0_LASTPC_BEFORE_EXCEPTION,
    #[doc = "0x74 - cpu status register"]
    pub c0re_0_debug_mode: C0RE_0_DEBUG_MODE,
    #[doc = "0x78 - clock register"]
    pub clock_gate: CLOCK_GATE,
    _reserved31: [u8; 0x0380],
    #[doc = "0x3fc - version register"]
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
#[doc = "CORE_0_AREA_DRAM0_0_MIN (rw) register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_0_MIN_SPEC>`"]
pub type CORE_0_AREA_DRAM0_0_MIN =
    crate::Reg<core_0_area_dram0_0_min::CORE_0_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "core0 dram0 region0 addr configuration register"]
pub mod core_0_area_dram0_0_min;
#[doc = "CORE_0_AREA_DRAM0_0_MAX (rw) register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_0_MAX_SPEC>`"]
pub type CORE_0_AREA_DRAM0_0_MAX =
    crate::Reg<core_0_area_dram0_0_max::CORE_0_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "core0 dram0 region0 addr configuration register"]
pub mod core_0_area_dram0_0_max;
#[doc = "CORE_0_AREA_DRAM0_1_MIN (rw) register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_1_MIN_SPEC>`"]
pub type CORE_0_AREA_DRAM0_1_MIN =
    crate::Reg<core_0_area_dram0_1_min::CORE_0_AREA_DRAM0_1_MIN_SPEC>;
#[doc = "core0 dram0 region1 addr configuration register"]
pub mod core_0_area_dram0_1_min;
#[doc = "CORE_0_AREA_DRAM0_1_MAX (rw) register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_1_MAX_SPEC>`"]
pub type CORE_0_AREA_DRAM0_1_MAX =
    crate::Reg<core_0_area_dram0_1_max::CORE_0_AREA_DRAM0_1_MAX_SPEC>;
#[doc = "core0 dram0 region1 addr configuration register"]
pub mod core_0_area_dram0_1_max;
#[doc = "CORE_0_AREA_PIF_0_MIN (rw) register accessor: an alias for `Reg<CORE_0_AREA_PIF_0_MIN_SPEC>`"]
pub type CORE_0_AREA_PIF_0_MIN = crate::Reg<core_0_area_pif_0_min::CORE_0_AREA_PIF_0_MIN_SPEC>;
#[doc = "core0 PIF region0 addr configuration register"]
pub mod core_0_area_pif_0_min;
#[doc = "CORE_0_AREA_PIF_0_MAX (rw) register accessor: an alias for `Reg<CORE_0_AREA_PIF_0_MAX_SPEC>`"]
pub type CORE_0_AREA_PIF_0_MAX = crate::Reg<core_0_area_pif_0_max::CORE_0_AREA_PIF_0_MAX_SPEC>;
#[doc = "core0 PIF region0 addr configuration register"]
pub mod core_0_area_pif_0_max;
#[doc = "CORE_0_AREA_PIF_1_MIN (rw) register accessor: an alias for `Reg<CORE_0_AREA_PIF_1_MIN_SPEC>`"]
pub type CORE_0_AREA_PIF_1_MIN = crate::Reg<core_0_area_pif_1_min::CORE_0_AREA_PIF_1_MIN_SPEC>;
#[doc = "core0 PIF region1 addr configuration register"]
pub mod core_0_area_pif_1_min;
#[doc = "CORE_0_AREA_PIF_1_MAX (rw) register accessor: an alias for `Reg<CORE_0_AREA_PIF_1_MAX_SPEC>`"]
pub type CORE_0_AREA_PIF_1_MAX = crate::Reg<core_0_area_pif_1_max::CORE_0_AREA_PIF_1_MAX_SPEC>;
#[doc = "core0 PIF region1 addr configuration register"]
pub mod core_0_area_pif_1_max;
#[doc = "CORE_0_AREA_PC (r) register accessor: an alias for `Reg<CORE_0_AREA_PC_SPEC>`"]
pub type CORE_0_AREA_PC = crate::Reg<core_0_area_pc::CORE_0_AREA_PC_SPEC>;
#[doc = "core0 area pc status register"]
pub mod core_0_area_pc;
#[doc = "CORE_0_AREA_SP (r) register accessor: an alias for `Reg<CORE_0_AREA_SP_SPEC>`"]
pub type CORE_0_AREA_SP = crate::Reg<core_0_area_sp::CORE_0_AREA_SP_SPEC>;
#[doc = "core0 area sp status register"]
pub mod core_0_area_sp;
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
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_0 (r) register accessor: an alias for `Reg<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_iram0_exception_monitor_0::CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "exception monitor status register0"]
pub mod core_0_iram0_exception_monitor_0;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_1 (r) register accessor: an alias for `Reg<CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_iram0_exception_monitor_1::CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "exception monitor status register1"]
pub mod core_0_iram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_0 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_dram0_exception_monitor_0::CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "exception monitor status register2"]
pub mod core_0_dram0_exception_monitor_0;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_1 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_dram0_exception_monitor_1::CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "exception monitor status register3"]
pub mod core_0_dram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<core_0_dram0_exception_monitor_2::CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "exception monitor status register4"]
pub mod core_0_dram0_exception_monitor_2;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_3 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<core_0_dram0_exception_monitor_3::CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "exception monitor status register5"]
pub mod core_0_dram0_exception_monitor_3;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "exception monitor status register6"]
pub mod core_x_iram0_dram0_exception_monitor_0;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_1::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "exception monitor status register7"]
pub mod core_x_iram0_dram0_exception_monitor_1;
#[doc = "C0RE_0_LASTPC_BEFORE_EXCEPTION (r) register accessor: an alias for `Reg<C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>`"]
pub type C0RE_0_LASTPC_BEFORE_EXCEPTION =
    crate::Reg<c0re_0_lastpc_before_exception::C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "cpu status register"]
pub mod c0re_0_lastpc_before_exception;
#[doc = "C0RE_0_DEBUG_MODE (r) register accessor: an alias for `Reg<C0RE_0_DEBUG_MODE_SPEC>`"]
pub type C0RE_0_DEBUG_MODE = crate::Reg<c0re_0_debug_mode::C0RE_0_DEBUG_MODE_SPEC>;
#[doc = "cpu status register"]
pub mod c0re_0_debug_mode;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
