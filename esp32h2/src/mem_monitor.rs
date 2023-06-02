#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - log config regsiter"]
    pub log_setting: LOG_SETTING,
    #[doc = "0x04 - check data regsiter"]
    pub log_check_data: LOG_CHECK_DATA,
    #[doc = "0x08 - check data mask register"]
    pub log_data_mask: LOG_DATA_MASK,
    #[doc = "0x0c - log boundary regsiter"]
    pub log_min: LOG_MIN,
    #[doc = "0x10 - log boundary regsiter"]
    pub log_max: LOG_MAX,
    #[doc = "0x14 - log message store range register"]
    pub log_mem_start: LOG_MEM_START,
    #[doc = "0x18 - log message store range register"]
    pub log_mem_end: LOG_MEM_END,
    #[doc = "0x1c - current writing address."]
    pub log_mem_current_addr: LOG_MEM_CURRENT_ADDR,
    #[doc = "0x20 - writing address update"]
    pub log_mem_addr_update: LOG_MEM_ADDR_UPDATE,
    #[doc = "0x24 - full flag status register"]
    pub log_mem_full_flag: LOG_MEM_FULL_FLAG,
    #[doc = "0x28 - clock gate force on register"]
    pub clock_gate: CLOCK_GATE,
    _reserved11: [u8; 0x03d0],
    #[doc = "0x3fc - version register"]
    pub date: DATE,
}
#[doc = "LOG_SETTING (rw) register accessor: an alias for `Reg<LOG_SETTING_SPEC>`"]
pub type LOG_SETTING = crate::Reg<log_setting::LOG_SETTING_SPEC>;
#[doc = "log config regsiter"]
pub mod log_setting;
#[doc = "LOG_CHECK_DATA (rw) register accessor: an alias for `Reg<LOG_CHECK_DATA_SPEC>`"]
pub type LOG_CHECK_DATA = crate::Reg<log_check_data::LOG_CHECK_DATA_SPEC>;
#[doc = "check data regsiter"]
pub mod log_check_data;
#[doc = "LOG_DATA_MASK (rw) register accessor: an alias for `Reg<LOG_DATA_MASK_SPEC>`"]
pub type LOG_DATA_MASK = crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>;
#[doc = "check data mask register"]
pub mod log_data_mask;
#[doc = "LOG_MIN (rw) register accessor: an alias for `Reg<LOG_MIN_SPEC>`"]
pub type LOG_MIN = crate::Reg<log_min::LOG_MIN_SPEC>;
#[doc = "log boundary regsiter"]
pub mod log_min;
#[doc = "LOG_MAX (rw) register accessor: an alias for `Reg<LOG_MAX_SPEC>`"]
pub type LOG_MAX = crate::Reg<log_max::LOG_MAX_SPEC>;
#[doc = "log boundary regsiter"]
pub mod log_max;
#[doc = "LOG_MEM_START (rw) register accessor: an alias for `Reg<LOG_MEM_START_SPEC>`"]
pub type LOG_MEM_START = crate::Reg<log_mem_start::LOG_MEM_START_SPEC>;
#[doc = "log message store range register"]
pub mod log_mem_start;
#[doc = "LOG_MEM_END (rw) register accessor: an alias for `Reg<LOG_MEM_END_SPEC>`"]
pub type LOG_MEM_END = crate::Reg<log_mem_end::LOG_MEM_END_SPEC>;
#[doc = "log message store range register"]
pub mod log_mem_end;
#[doc = "LOG_MEM_CURRENT_ADDR (r) register accessor: an alias for `Reg<LOG_MEM_CURRENT_ADDR_SPEC>`"]
pub type LOG_MEM_CURRENT_ADDR = crate::Reg<log_mem_current_addr::LOG_MEM_CURRENT_ADDR_SPEC>;
#[doc = "current writing address."]
pub mod log_mem_current_addr;
#[doc = "LOG_MEM_ADDR_UPDATE (w) register accessor: an alias for `Reg<LOG_MEM_ADDR_UPDATE_SPEC>`"]
pub type LOG_MEM_ADDR_UPDATE = crate::Reg<log_mem_addr_update::LOG_MEM_ADDR_UPDATE_SPEC>;
#[doc = "writing address update"]
pub mod log_mem_addr_update;
#[doc = "LOG_MEM_FULL_FLAG (rw) register accessor: an alias for `Reg<LOG_MEM_FULL_FLAG_SPEC>`"]
pub type LOG_MEM_FULL_FLAG = crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>;
#[doc = "full flag status register"]
pub mod log_mem_full_flag;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gate force on register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
