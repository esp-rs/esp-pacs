#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu: [CPU; 2],
    core_x_iram0_dram0_exception_monitor: [CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR; 2],
    log_setting: LOG_SETTING,
    log_data_0: LOG_DATA_0,
    log_data_1: LOG_DATA_1,
    log_data_2: LOG_DATA_2,
    log_data_3: LOG_DATA_3,
    log_data_mask: LOG_DATA_MASK,
    log_min: LOG_MIN,
    log_max: LOG_MAX,
    log_mem_start: LOG_MEM_START,
    log_mem_end: LOG_MEM_END,
    log_mem_writing_addr: LOG_MEM_WRITING_ADDR,
    log_mem_full_flag: LOG_MEM_FULL_FLAG,
    _reserved14: [u8; 0xa4],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x120 - Cluster CPU%s, containing CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_DRAM0_EXCEPTION_MONITOR_4, CORE_?_DRAM0_EXCEPTION_MONITOR_5, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_MONTR_ENA, CORE_?_RCD_PDEBUGDATA, CORE_?_RCD_PDEBUGENABLE, CORE_?_RCD_PDEBUGINST, CORE_?_RCD_PDEBUGLS0ADDR, CORE_?_RCD_PDEBUGLS0DATA, CORE_?_RCD_PDEBUGLS0STAT, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSTATUS, CORE_?_RCD_RECORDING, CORE_?_RCD_SP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC, CORE_?_SP_UNSTABLE"]
    #[inline(always)]
    pub const fn cpu(&self, n: usize) -> &CPU {
        &self.cpu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x120 - Cluster CPU%s, containing CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_DRAM0_EXCEPTION_MONITOR_4, CORE_?_DRAM0_EXCEPTION_MONITOR_5, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_MONTR_ENA, CORE_?_RCD_PDEBUGDATA, CORE_?_RCD_PDEBUGENABLE, CORE_?_RCD_PDEBUGINST, CORE_?_RCD_PDEBUGLS0ADDR, CORE_?_RCD_PDEBUGLS0DATA, CORE_?_RCD_PDEBUGLS0STAT, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSTATUS, CORE_?_RCD_RECORDING, CORE_?_RCD_SP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC, CORE_?_SP_UNSTABLE"]
    #[inline(always)]
    pub fn cpu_iter(&self) -> impl Iterator<Item = &CPU> {
        self.cpu.iter()
    }
    #[doc = "0x120..0x128 - `¯\\_(ツ)_/¯`"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor(
        &self,
        n: usize,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR {
        &self.core_x_iram0_dram0_exception_monitor[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x128 - `¯\\_(ツ)_/¯`"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_exception_monitor_iter(
        &self,
    ) -> impl Iterator<Item = &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR> {
        self.core_x_iram0_dram0_exception_monitor.iter()
    }
    #[doc = "0x128 - log set register"]
    #[inline(always)]
    pub const fn log_setting(&self) -> &LOG_SETTING {
        &self.log_setting
    }
    #[doc = "0x12c - log check data register"]
    #[inline(always)]
    pub const fn log_data_0(&self) -> &LOG_DATA_0 {
        &self.log_data_0
    }
    #[doc = "0x130 - log check data register"]
    #[inline(always)]
    pub const fn log_data_1(&self) -> &LOG_DATA_1 {
        &self.log_data_1
    }
    #[doc = "0x134 - log check data register"]
    #[inline(always)]
    pub const fn log_data_2(&self) -> &LOG_DATA_2 {
        &self.log_data_2
    }
    #[doc = "0x138 - log check data register"]
    #[inline(always)]
    pub const fn log_data_3(&self) -> &LOG_DATA_3 {
        &self.log_data_3
    }
    #[doc = "0x13c - log check data mask register"]
    #[inline(always)]
    pub const fn log_data_mask(&self) -> &LOG_DATA_MASK {
        &self.log_data_mask
    }
    #[doc = "0x140 - log check region configuration register"]
    #[inline(always)]
    pub const fn log_min(&self) -> &LOG_MIN {
        &self.log_min
    }
    #[doc = "0x144 - log check region configuration register"]
    #[inline(always)]
    pub const fn log_max(&self) -> &LOG_MAX {
        &self.log_max
    }
    #[doc = "0x148 - log mem region configuration register"]
    #[inline(always)]
    pub const fn log_mem_start(&self) -> &LOG_MEM_START {
        &self.log_mem_start
    }
    #[doc = "0x14c - log mem region configuration register"]
    #[inline(always)]
    pub const fn log_mem_end(&self) -> &LOG_MEM_END {
        &self.log_mem_end
    }
    #[doc = "0x150 - log mem addr status register"]
    #[inline(always)]
    pub const fn log_mem_writing_addr(&self) -> &LOG_MEM_WRITING_ADDR {
        &self.log_mem_writing_addr
    }
    #[doc = "0x154 - log mem status register"]
    #[inline(always)]
    pub const fn log_mem_full_flag(&self) -> &LOG_MEM_FULL_FLAG {
        &self.log_mem_full_flag
    }
    #[doc = "0x1fc - version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "Cluster CPU%s, containing CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_DRAM0_EXCEPTION_MONITOR_4, CORE_?_DRAM0_EXCEPTION_MONITOR_5, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_MONTR_ENA, CORE_?_RCD_PDEBUGDATA, CORE_?_RCD_PDEBUGENABLE, CORE_?_RCD_PDEBUGINST, CORE_?_RCD_PDEBUGLS0ADDR, CORE_?_RCD_PDEBUGLS0DATA, CORE_?_RCD_PDEBUGLS0STAT, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSTATUS, CORE_?_RCD_RECORDING, CORE_?_RCD_SP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC, CORE_?_SP_UNSTABLE"]
pub use self::cpu::CPU;
#[doc = r"Cluster"]
#[doc = "Cluster CPU%s, containing CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_DRAM0_EXCEPTION_MONITOR_4, CORE_?_DRAM0_EXCEPTION_MONITOR_5, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_MONTR_ENA, CORE_?_RCD_PDEBUGDATA, CORE_?_RCD_PDEBUGENABLE, CORE_?_RCD_PDEBUGINST, CORE_?_RCD_PDEBUGLS0ADDR, CORE_?_RCD_PDEBUGLS0DATA, CORE_?_RCD_PDEBUGLS0STAT, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSTATUS, CORE_?_RCD_RECORDING, CORE_?_RCD_SP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC, CORE_?_SP_UNSTABLE"]
pub mod cpu;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR (rw) register accessor: `¯\\_(ツ)_/¯`\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR =
    crate::Reg<core_x_iram0_dram0_exception_monitor::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_SPEC>;
#[doc = "`¯\\_(ツ)_/¯`"]
pub mod core_x_iram0_dram0_exception_monitor;
#[doc = "LOG_SETTING (rw) register accessor: log set register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_setting::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_setting::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_setting`] module"]
pub type LOG_SETTING = crate::Reg<log_setting::LOG_SETTING_SPEC>;
#[doc = "log set register"]
pub mod log_setting;
#[doc = "LOG_DATA_0 (rw) register accessor: log check data register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_data_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_0`] module"]
pub type LOG_DATA_0 = crate::Reg<log_data_0::LOG_DATA_0_SPEC>;
#[doc = "log check data register"]
pub mod log_data_0;
#[doc = "LOG_DATA_1 (rw) register accessor: log check data register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_data_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_1`] module"]
pub type LOG_DATA_1 = crate::Reg<log_data_1::LOG_DATA_1_SPEC>;
#[doc = "log check data register"]
pub mod log_data_1;
#[doc = "LOG_DATA_2 (rw) register accessor: log check data register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_data_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_2`] module"]
pub type LOG_DATA_2 = crate::Reg<log_data_2::LOG_DATA_2_SPEC>;
#[doc = "log check data register"]
pub mod log_data_2;
#[doc = "LOG_DATA_3 (rw) register accessor: log check data register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_data_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_3`] module"]
pub type LOG_DATA_3 = crate::Reg<log_data_3::LOG_DATA_3_SPEC>;
#[doc = "log check data register"]
pub mod log_data_3;
#[doc = "LOG_DATA_MASK (rw) register accessor: log check data mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_data_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_mask`] module"]
pub type LOG_DATA_MASK = crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>;
#[doc = "log check data mask register"]
pub mod log_data_mask;
#[doc = "LOG_MIN (rw) register accessor: log check region configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_min`] module"]
pub type LOG_MIN = crate::Reg<log_min::LOG_MIN_SPEC>;
#[doc = "log check region configuration register"]
pub mod log_min;
#[doc = "LOG_MAX (rw) register accessor: log check region configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_max`] module"]
pub type LOG_MAX = crate::Reg<log_max::LOG_MAX_SPEC>;
#[doc = "log check region configuration register"]
pub mod log_max;
#[doc = "LOG_MEM_START (rw) register accessor: log mem region configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_start`] module"]
pub type LOG_MEM_START = crate::Reg<log_mem_start::LOG_MEM_START_SPEC>;
#[doc = "log mem region configuration register"]
pub mod log_mem_start;
#[doc = "LOG_MEM_END (rw) register accessor: log mem region configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_end`] module"]
pub type LOG_MEM_END = crate::Reg<log_mem_end::LOG_MEM_END_SPEC>;
#[doc = "log mem region configuration register"]
pub mod log_mem_end;
#[doc = "LOG_MEM_WRITING_ADDR (r) register accessor: log mem addr status register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_writing_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_writing_addr`] module"]
pub type LOG_MEM_WRITING_ADDR = crate::Reg<log_mem_writing_addr::LOG_MEM_WRITING_ADDR_SPEC>;
#[doc = "log mem addr status register"]
pub mod log_mem_writing_addr;
#[doc = "LOG_MEM_FULL_FLAG (rw) register accessor: log mem status register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_full_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_full_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_full_flag`] module"]
pub type LOG_MEM_FULL_FLAG = crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>;
#[doc = "log mem status register"]
pub mod log_mem_full_flag;
#[doc = "DATE (rw) register accessor: version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
