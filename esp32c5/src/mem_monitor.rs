#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    log_setting: LOG_SETTING,
    log_setting1: LOG_SETTING1,
    log_check_data: LOG_CHECK_DATA,
    log_data_mask: LOG_DATA_MASK,
    log_min: LOG_MIN,
    log_max: LOG_MAX,
    log_mon_addr_update_0: LOG_MON_ADDR_UPDATE_0,
    log_mon_addr_update_1: LOG_MON_ADDR_UPDATE_1,
    log_mem_start: LOG_MEM_START,
    log_mem_end: LOG_MEM_END,
    log_mem_current_addr: LOG_MEM_CURRENT_ADDR,
    log_mem_addr_update: LOG_MEM_ADDR_UPDATE,
    log_mem_full_flag: LOG_MEM_FULL_FLAG,
    clock_gate: CLOCK_GATE,
    _reserved14: [u8; 0x03c4],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Bus access logging configuration register"]
    #[inline(always)]
    pub const fn log_setting(&self) -> &LOG_SETTING {
        &self.log_setting
    }
    #[doc = "0x04 - Bus access logging configuration register"]
    #[inline(always)]
    pub const fn log_setting1(&self) -> &LOG_SETTING1 {
        &self.log_setting1
    }
    #[doc = "0x08 - Configures monitored data in Bus access logging"]
    #[inline(always)]
    pub const fn log_check_data(&self) -> &LOG_CHECK_DATA {
        &self.log_check_data
    }
    #[doc = "0x0c - Configures masked data in Bus access logging"]
    #[inline(always)]
    pub const fn log_data_mask(&self) -> &LOG_DATA_MASK {
        &self.log_data_mask
    }
    #[doc = "0x10 - Configures monitored address space in Bus access logging"]
    #[inline(always)]
    pub const fn log_min(&self) -> &LOG_MIN {
        &self.log_min
    }
    #[doc = "0x14 - Configures monitored address space in Bus access logging"]
    #[inline(always)]
    pub const fn log_max(&self) -> &LOG_MAX {
        &self.log_max
    }
    #[doc = "0x18 - Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the certain master."]
    #[inline(always)]
    pub const fn log_mon_addr_update_0(&self) -> &LOG_MON_ADDR_UPDATE_0 {
        &self.log_mon_addr_update_0
    }
    #[doc = "0x1c - Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the certain master."]
    #[inline(always)]
    pub const fn log_mon_addr_update_1(&self) -> &LOG_MON_ADDR_UPDATE_1 {
        &self.log_mon_addr_update_1
    }
    #[doc = "0x20 - Configures the starting address of the storage memory for recorded data"]
    #[inline(always)]
    pub const fn log_mem_start(&self) -> &LOG_MEM_START {
        &self.log_mem_start
    }
    #[doc = "0x24 - Configures the end address of the storage memory for recorded data"]
    #[inline(always)]
    pub const fn log_mem_end(&self) -> &LOG_MEM_END {
        &self.log_mem_end
    }
    #[doc = "0x28 - Represents the address for the next write"]
    #[inline(always)]
    pub const fn log_mem_current_addr(&self) -> &LOG_MEM_CURRENT_ADDR {
        &self.log_mem_current_addr
    }
    #[doc = "0x2c - Updates the address for the next write with the starting address for the recorded data"]
    #[inline(always)]
    pub const fn log_mem_addr_update(&self) -> &LOG_MEM_ADDR_UPDATE {
        &self.log_mem_addr_update
    }
    #[doc = "0x30 - Logging overflow status register"]
    #[inline(always)]
    pub const fn log_mem_full_flag(&self) -> &LOG_MEM_FULL_FLAG {
        &self.log_mem_full_flag
    }
    #[doc = "0x34 - Register clock control"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "LOG_SETTING (rw) register accessor: Bus access logging configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_setting::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_setting::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_setting`] module"]
pub type LOG_SETTING = crate::Reg<log_setting::LOG_SETTING_SPEC>;
#[doc = "Bus access logging configuration register"]
pub mod log_setting;
#[doc = "LOG_SETTING1 (rw) register accessor: Bus access logging configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_setting1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_setting1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_setting1`] module"]
pub type LOG_SETTING1 = crate::Reg<log_setting1::LOG_SETTING1_SPEC>;
#[doc = "Bus access logging configuration register"]
pub mod log_setting1;
#[doc = "LOG_CHECK_DATA (rw) register accessor: Configures monitored data in Bus access logging\n\nYou can [`read`](crate::Reg::read) this register and get [`log_check_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_check_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_check_data`] module"]
pub type LOG_CHECK_DATA = crate::Reg<log_check_data::LOG_CHECK_DATA_SPEC>;
#[doc = "Configures monitored data in Bus access logging"]
pub mod log_check_data;
#[doc = "LOG_DATA_MASK (rw) register accessor: Configures masked data in Bus access logging\n\nYou can [`read`](crate::Reg::read) this register and get [`log_data_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_mask`] module"]
pub type LOG_DATA_MASK = crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>;
#[doc = "Configures masked data in Bus access logging"]
pub mod log_data_mask;
#[doc = "LOG_MIN (rw) register accessor: Configures monitored address space in Bus access logging\n\nYou can [`read`](crate::Reg::read) this register and get [`log_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_min`] module"]
pub type LOG_MIN = crate::Reg<log_min::LOG_MIN_SPEC>;
#[doc = "Configures monitored address space in Bus access logging"]
pub mod log_min;
#[doc = "LOG_MAX (rw) register accessor: Configures monitored address space in Bus access logging\n\nYou can [`read`](crate::Reg::read) this register and get [`log_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_max`] module"]
pub type LOG_MAX = crate::Reg<log_max::LOG_MAX_SPEC>;
#[doc = "Configures monitored address space in Bus access logging"]
pub mod log_max;
#[doc = "LOG_MON_ADDR_UPDATE_0 (w) register accessor: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the certain master.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mon_addr_update_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mon_addr_update_0`] module"]
pub type LOG_MON_ADDR_UPDATE_0 = crate::Reg<log_mon_addr_update_0::LOG_MON_ADDR_UPDATE_0_SPEC>;
#[doc = "Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the certain master."]
pub mod log_mon_addr_update_0;
#[doc = "LOG_MON_ADDR_UPDATE_1 (w) register accessor: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the certain master.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mon_addr_update_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mon_addr_update_1`] module"]
pub type LOG_MON_ADDR_UPDATE_1 = crate::Reg<log_mon_addr_update_1::LOG_MON_ADDR_UPDATE_1_SPEC>;
#[doc = "Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the certain master."]
pub mod log_mon_addr_update_1;
#[doc = "LOG_MEM_START (rw) register accessor: Configures the starting address of the storage memory for recorded data\n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_start`] module"]
pub type LOG_MEM_START = crate::Reg<log_mem_start::LOG_MEM_START_SPEC>;
#[doc = "Configures the starting address of the storage memory for recorded data"]
pub mod log_mem_start;
#[doc = "LOG_MEM_END (rw) register accessor: Configures the end address of the storage memory for recorded data\n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_end`] module"]
pub type LOG_MEM_END = crate::Reg<log_mem_end::LOG_MEM_END_SPEC>;
#[doc = "Configures the end address of the storage memory for recorded data"]
pub mod log_mem_end;
#[doc = "LOG_MEM_CURRENT_ADDR (r) register accessor: Represents the address for the next write\n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_current_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_current_addr`] module"]
pub type LOG_MEM_CURRENT_ADDR = crate::Reg<log_mem_current_addr::LOG_MEM_CURRENT_ADDR_SPEC>;
#[doc = "Represents the address for the next write"]
pub mod log_mem_current_addr;
#[doc = "LOG_MEM_ADDR_UPDATE (w) register accessor: Updates the address for the next write with the starting address for the recorded data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_addr_update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_addr_update`] module"]
pub type LOG_MEM_ADDR_UPDATE = crate::Reg<log_mem_addr_update::LOG_MEM_ADDR_UPDATE_SPEC>;
#[doc = "Updates the address for the next write with the starting address for the recorded data"]
pub mod log_mem_addr_update;
#[doc = "LOG_MEM_FULL_FLAG (rw) register accessor: Logging overflow status register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_full_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_full_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_full_flag`] module"]
pub type LOG_MEM_FULL_FLAG = crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>;
#[doc = "Logging overflow status register"]
pub mod log_mem_full_flag;
#[doc = "CLOCK_GATE (rw) register accessor: Register clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Register clock control"]
pub mod clock_gate;
pub use crate::aes::date;
pub use crate::aes::DATE;
