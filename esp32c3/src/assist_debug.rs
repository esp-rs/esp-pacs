#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - ASSIST_DEBUG_C0RE_0_MONTR_ENA_REG"]
    pub core_0_montr_ena: CORE_0_MONTR_ENA,
    #[doc = "0x04 - ASSIST_DEBUG_CORE_0_INTR_RAW_REG"]
    pub core_0_intr_raw: CORE_0_INTR_RAW,
    #[doc = "0x08 - ASSIST_DEBUG_CORE_0_INTR_ENA_REG"]
    pub core_0_intr_ena: CORE_0_INTR_ENA,
    #[doc = "0x0c - ASSIST_DEBUG_CORE_0_INTR_CLR_REG"]
    pub core_0_intr_clr: CORE_0_INTR_CLR,
    #[doc = "0x10 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN_REG"]
    pub core_0_area_dram0_0_min: CORE_0_AREA_DRAM0_0_MIN,
    #[doc = "0x14 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_REG"]
    pub core_0_area_dram0_0_max: CORE_0_AREA_DRAM0_0_MAX,
    #[doc = "0x18 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN_REG"]
    pub core_0_area_dram0_1_min: CORE_0_AREA_DRAM0_1_MIN,
    #[doc = "0x1c - ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX_REG"]
    pub core_0_area_dram0_1_max: CORE_0_AREA_DRAM0_1_MAX,
    #[doc = "0x20 - ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN_REG"]
    pub core_0_area_pif_0_min: CORE_0_AREA_PIF_0_MIN,
    #[doc = "0x24 - ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX_REG"]
    pub core_0_area_pif_0_max: CORE_0_AREA_PIF_0_MAX,
    #[doc = "0x28 - ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN_REG"]
    pub core_0_area_pif_1_min: CORE_0_AREA_PIF_1_MIN,
    #[doc = "0x2c - ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX_REG"]
    pub core_0_area_pif_1_max: CORE_0_AREA_PIF_1_MAX,
    #[doc = "0x30 - ASSIST_DEBUG_CORE_0_AREA_PC_REG"]
    pub core_0_area_pc: CORE_0_AREA_PC,
    #[doc = "0x34 - ASSIST_DEBUG_CORE_0_AREA_SP_REG"]
    pub core_0_area_sp: CORE_0_AREA_SP,
    #[doc = "0x38 - ASSIST_DEBUG_CORE_0_SP_MIN_REG"]
    pub core_0_sp_min: CORE_0_SP_MIN,
    #[doc = "0x3c - ASSIST_DEBUG_CORE_0_SP_MAX_REG"]
    pub core_0_sp_max: CORE_0_SP_MAX,
    #[doc = "0x40 - ASSIST_DEBUG_CORE_0_SP_PC_REG"]
    pub core_0_sp_pc: CORE_0_SP_PC,
    #[doc = "0x44 - ASSIST_DEBUG_CORE_0_RCD_EN_REG"]
    pub core_0_rcd_en: CORE_0_RCD_EN,
    #[doc = "0x48 - ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG"]
    pub core_0_rcd_pdebugpc: CORE_0_RCD_PDEBUGPC,
    #[doc = "0x4c - ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
    pub core_0_rcd_pdebugsp: CORE_0_RCD_PDEBUGSP,
    #[doc = "0x50 - ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
    pub core_0_iram0_exception_monitor_0: CORE_0_IRAM0_EXCEPTION_MONITOR_0,
    #[doc = "0x54 - ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1_REG"]
    pub core_0_iram0_exception_monitor_1: CORE_0_IRAM0_EXCEPTION_MONITOR_1,
    #[doc = "0x58 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0_REG"]
    pub core_0_dram0_exception_monitor_0: CORE_0_DRAM0_EXCEPTION_MONITOR_0,
    #[doc = "0x5c - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
    pub core_0_dram0_exception_monitor_1: CORE_0_DRAM0_EXCEPTION_MONITOR_1,
    #[doc = "0x60 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
    pub core_0_dram0_exception_monitor_2: CORE_0_DRAM0_EXCEPTION_MONITOR_2,
    #[doc = "0x64 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3_REG"]
    pub core_0_dram0_exception_monitor_3: CORE_0_DRAM0_EXCEPTION_MONITOR_3,
    #[doc = "0x68 - ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_REG"]
    pub core_x_iram0_dram0_exception_monitor_0: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0,
    #[doc = "0x6c - ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_REG"]
    pub core_x_iram0_dram0_exception_monitor_1: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1,
    #[doc = "0x70 - ASSIST_DEBUG_LOG_SETTING"]
    pub log_setting: LOG_SETTING,
    #[doc = "0x74 - ASSIST_DEBUG_LOG_DATA_0_REG"]
    pub log_data_0: LOG_DATA_0,
    #[doc = "0x78 - ASSIST_DEBUG_LOG_DATA_MASK_REG"]
    pub log_data_mask: LOG_DATA_MASK,
    #[doc = "0x7c - ASSIST_DEBUG_LOG_MIN_REG"]
    pub log_min: LOG_MIN,
    #[doc = "0x80 - ASSIST_DEBUG_LOG_MAX_REG"]
    pub log_max: LOG_MAX,
    #[doc = "0x84 - ASSIST_DEBUG_LOG_MEM_START_REG"]
    pub log_mem_start: LOG_MEM_START,
    #[doc = "0x88 - ASSIST_DEBUG_LOG_MEM_END_REG"]
    pub log_mem_end: LOG_MEM_END,
    #[doc = "0x8c - ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_REG"]
    pub log_mem_writing_addr: LOG_MEM_WRITING_ADDR,
    #[doc = "0x90 - ASSIST_DEBUG_LOG_MEM_FULL_FLAG_REG"]
    pub log_mem_full_flag: LOG_MEM_FULL_FLAG,
    #[doc = "0x94 - ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
    pub c0re_0_lastpc_before_exception: C0RE_0_LASTPC_BEFORE_EXCEPTION,
    #[doc = "0x98 - ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
    pub c0re_0_debug_mode: C0RE_0_DEBUG_MODE,
    _reserved39: [u8; 0x0160],
    #[doc = "0x1fc - ASSIST_DEBUG_DATE_REG"]
    pub date: DATE,
}
#[doc = "CORE_0_MONTR_ENA (rw) register accessor: an alias for `Reg<CORE_0_MONTR_ENA_SPEC>`"]
pub type CORE_0_MONTR_ENA = crate::Reg<core_0_montr_ena::CORE_0_MONTR_ENA_SPEC>;
#[doc = "ASSIST_DEBUG_C0RE_0_MONTR_ENA_REG"]
pub mod core_0_montr_ena;
#[doc = "CORE_0_INTR_RAW (r) register accessor: an alias for `Reg<CORE_0_INTR_RAW_SPEC>`"]
pub type CORE_0_INTR_RAW = crate::Reg<core_0_intr_raw::CORE_0_INTR_RAW_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RAW_REG"]
pub mod core_0_intr_raw;
#[doc = "CORE_0_INTR_ENA (rw) register accessor: an alias for `Reg<CORE_0_INTR_ENA_SPEC>`"]
pub type CORE_0_INTR_ENA = crate::Reg<core_0_intr_ena::CORE_0_INTR_ENA_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_ENA_REG"]
pub mod core_0_intr_ena;
#[doc = "CORE_0_INTR_CLR (rw) register accessor: an alias for `Reg<CORE_0_INTR_CLR_SPEC>`"]
pub type CORE_0_INTR_CLR = crate::Reg<core_0_intr_clr::CORE_0_INTR_CLR_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_CLR_REG"]
pub mod core_0_intr_clr;
#[doc = "CORE_0_AREA_DRAM0_0_MIN (rw) register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_0_MIN_SPEC>`"]
pub type CORE_0_AREA_DRAM0_0_MIN =
    crate::Reg<core_0_area_dram0_0_min::CORE_0_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN_REG"]
pub mod core_0_area_dram0_0_min;
#[doc = "CORE_0_AREA_DRAM0_0_MAX (rw) register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_0_MAX_SPEC>`"]
pub type CORE_0_AREA_DRAM0_0_MAX =
    crate::Reg<core_0_area_dram0_0_max::CORE_0_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_REG"]
pub mod core_0_area_dram0_0_max;
#[doc = "CORE_0_AREA_DRAM0_1_MIN (rw) register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_1_MIN_SPEC>`"]
pub type CORE_0_AREA_DRAM0_1_MIN =
    crate::Reg<core_0_area_dram0_1_min::CORE_0_AREA_DRAM0_1_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN_REG"]
pub mod core_0_area_dram0_1_min;
#[doc = "CORE_0_AREA_DRAM0_1_MAX (rw) register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_1_MAX_SPEC>`"]
pub type CORE_0_AREA_DRAM0_1_MAX =
    crate::Reg<core_0_area_dram0_1_max::CORE_0_AREA_DRAM0_1_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX_REG"]
pub mod core_0_area_dram0_1_max;
#[doc = "CORE_0_AREA_PIF_0_MIN (rw) register accessor: an alias for `Reg<CORE_0_AREA_PIF_0_MIN_SPEC>`"]
pub type CORE_0_AREA_PIF_0_MIN = crate::Reg<core_0_area_pif_0_min::CORE_0_AREA_PIF_0_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN_REG"]
pub mod core_0_area_pif_0_min;
#[doc = "CORE_0_AREA_PIF_0_MAX (rw) register accessor: an alias for `Reg<CORE_0_AREA_PIF_0_MAX_SPEC>`"]
pub type CORE_0_AREA_PIF_0_MAX = crate::Reg<core_0_area_pif_0_max::CORE_0_AREA_PIF_0_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX_REG"]
pub mod core_0_area_pif_0_max;
#[doc = "CORE_0_AREA_PIF_1_MIN (rw) register accessor: an alias for `Reg<CORE_0_AREA_PIF_1_MIN_SPEC>`"]
pub type CORE_0_AREA_PIF_1_MIN = crate::Reg<core_0_area_pif_1_min::CORE_0_AREA_PIF_1_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN_REG"]
pub mod core_0_area_pif_1_min;
#[doc = "CORE_0_AREA_PIF_1_MAX (rw) register accessor: an alias for `Reg<CORE_0_AREA_PIF_1_MAX_SPEC>`"]
pub type CORE_0_AREA_PIF_1_MAX = crate::Reg<core_0_area_pif_1_max::CORE_0_AREA_PIF_1_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX_REG"]
pub mod core_0_area_pif_1_max;
#[doc = "CORE_0_AREA_PC (r) register accessor: an alias for `Reg<CORE_0_AREA_PC_SPEC>`"]
pub type CORE_0_AREA_PC = crate::Reg<core_0_area_pc::CORE_0_AREA_PC_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PC_REG"]
pub mod core_0_area_pc;
#[doc = "CORE_0_AREA_SP (r) register accessor: an alias for `Reg<CORE_0_AREA_SP_SPEC>`"]
pub type CORE_0_AREA_SP = crate::Reg<core_0_area_sp::CORE_0_AREA_SP_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_SP_REG"]
pub mod core_0_area_sp;
#[doc = "CORE_0_SP_MIN (rw) register accessor: an alias for `Reg<CORE_0_SP_MIN_SPEC>`"]
pub type CORE_0_SP_MIN = crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_SP_MIN_REG"]
pub mod core_0_sp_min;
#[doc = "CORE_0_SP_MAX (rw) register accessor: an alias for `Reg<CORE_0_SP_MAX_SPEC>`"]
pub type CORE_0_SP_MAX = crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_SP_MAX_REG"]
pub mod core_0_sp_max;
#[doc = "CORE_0_SP_PC (r) register accessor: an alias for `Reg<CORE_0_SP_PC_SPEC>`"]
pub type CORE_0_SP_PC = crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_SP_PC_REG"]
pub mod core_0_sp_pc;
#[doc = "CORE_0_RCD_EN (rw) register accessor: an alias for `Reg<CORE_0_RCD_EN_SPEC>`"]
pub type CORE_0_RCD_EN = crate::Reg<core_0_rcd_en::CORE_0_RCD_EN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_EN_REG"]
pub mod core_0_rcd_en;
#[doc = "CORE_0_RCD_PDEBUGPC (r) register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGPC_SPEC>`"]
pub type CORE_0_RCD_PDEBUGPC = crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG"]
pub mod core_0_rcd_pdebugpc;
#[doc = "CORE_0_RCD_PDEBUGSP (r) register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGSP_SPEC>`"]
pub type CORE_0_RCD_PDEBUGSP = crate::Reg<core_0_rcd_pdebugsp::CORE_0_RCD_PDEBUGSP_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
pub mod core_0_rcd_pdebugsp;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_0 (r) register accessor: an alias for `Reg<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_iram0_exception_monitor_0::CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
pub mod core_0_iram0_exception_monitor_0;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_1 (r) register accessor: an alias for `Reg<CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_iram0_exception_monitor_1::CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod core_0_iram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_0 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_dram0_exception_monitor_0::CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0_REG"]
pub mod core_0_dram0_exception_monitor_0;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_1 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_dram0_exception_monitor_1::CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod core_0_dram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<core_0_dram0_exception_monitor_2::CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod core_0_dram0_exception_monitor_2;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_3 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<core_0_dram0_exception_monitor_3::CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3_REG"]
pub mod core_0_dram0_exception_monitor_3;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_REG"]
pub mod core_x_iram0_dram0_exception_monitor_0;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_1::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod core_x_iram0_dram0_exception_monitor_1;
#[doc = "LOG_SETTING (rw) register accessor: an alias for `Reg<LOG_SETTING_SPEC>`"]
pub type LOG_SETTING = crate::Reg<log_setting::LOG_SETTING_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_SETTING"]
pub mod log_setting;
#[doc = "LOG_DATA_0 (rw) register accessor: an alias for `Reg<LOG_DATA_0_SPEC>`"]
pub type LOG_DATA_0 = crate::Reg<log_data_0::LOG_DATA_0_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_DATA_0_REG"]
pub mod log_data_0;
#[doc = "LOG_DATA_MASK (rw) register accessor: an alias for `Reg<LOG_DATA_MASK_SPEC>`"]
pub type LOG_DATA_MASK = crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_DATA_MASK_REG"]
pub mod log_data_mask;
#[doc = "LOG_MIN (rw) register accessor: an alias for `Reg<LOG_MIN_SPEC>`"]
pub type LOG_MIN = crate::Reg<log_min::LOG_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MIN_REG"]
pub mod log_min;
#[doc = "LOG_MAX (rw) register accessor: an alias for `Reg<LOG_MAX_SPEC>`"]
pub type LOG_MAX = crate::Reg<log_max::LOG_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MAX_REG"]
pub mod log_max;
#[doc = "LOG_MEM_START (rw) register accessor: an alias for `Reg<LOG_MEM_START_SPEC>`"]
pub type LOG_MEM_START = crate::Reg<log_mem_start::LOG_MEM_START_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_START_REG"]
pub mod log_mem_start;
#[doc = "LOG_MEM_END (rw) register accessor: an alias for `Reg<LOG_MEM_END_SPEC>`"]
pub type LOG_MEM_END = crate::Reg<log_mem_end::LOG_MEM_END_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_END_REG"]
pub mod log_mem_end;
#[doc = "LOG_MEM_WRITING_ADDR (r) register accessor: an alias for `Reg<LOG_MEM_WRITING_ADDR_SPEC>`"]
pub type LOG_MEM_WRITING_ADDR = crate::Reg<log_mem_writing_addr::LOG_MEM_WRITING_ADDR_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_REG"]
pub mod log_mem_writing_addr;
#[doc = "LOG_MEM_FULL_FLAG (rw) register accessor: an alias for `Reg<LOG_MEM_FULL_FLAG_SPEC>`"]
pub type LOG_MEM_FULL_FLAG = crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_FULL_FLAG_REG"]
pub mod log_mem_full_flag;
#[doc = "C0RE_0_LASTPC_BEFORE_EXCEPTION (r) register accessor: an alias for `Reg<C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>`"]
pub type C0RE_0_LASTPC_BEFORE_EXCEPTION =
    crate::Reg<c0re_0_lastpc_before_exception::C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
pub mod c0re_0_lastpc_before_exception;
#[doc = "C0RE_0_DEBUG_MODE (r) register accessor: an alias for `Reg<C0RE_0_DEBUG_MODE_SPEC>`"]
pub type C0RE_0_DEBUG_MODE = crate::Reg<c0re_0_debug_mode::C0RE_0_DEBUG_MODE_SPEC>;
#[doc = "ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
pub mod c0re_0_debug_mode;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "ASSIST_DEBUG_DATE_REG"]
pub mod date;
