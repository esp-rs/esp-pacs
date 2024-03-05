#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    core_0_montr_ena: CORE_0_MONTR_ENA,
    core_0_intr_raw: CORE_0_INTR_RAW,
    core_0_intr_ena: CORE_0_INTR_ENA,
    core_0_intr_clr: CORE_0_INTR_CLR,
    core_0_area_dram0_0_min: CORE_0_AREA_DRAM0_0_MIN,
    core_0_area_dram0_0_max: CORE_0_AREA_DRAM0_0_MAX,
    core_0_area_dram0_1_min: CORE_0_AREA_DRAM0_1_MIN,
    core_0_area_dram0_1_max: CORE_0_AREA_DRAM0_1_MAX,
    core_0_area_pif_0_min: CORE_0_AREA_PIF_0_MIN,
    core_0_area_pif_0_max: CORE_0_AREA_PIF_0_MAX,
    core_0_area_pif_1_min: CORE_0_AREA_PIF_1_MIN,
    core_0_area_pif_1_max: CORE_0_AREA_PIF_1_MAX,
    core_0_area_pc: CORE_0_AREA_PC,
    core_0_area_sp: CORE_0_AREA_SP,
    core_0_sp_min: CORE_0_SP_MIN,
    core_0_sp_max: CORE_0_SP_MAX,
    core_0_sp_pc: CORE_0_SP_PC,
    core_0_rcd_en: CORE_0_RCD_EN,
    core_0_rcd_pdebugpc: CORE_0_RCD_PDEBUGPC,
    core_0_rcd_pdebugsp: CORE_0_RCD_PDEBUGSP,
    core_0_iram0_exception_monitor_0: CORE_0_IRAM0_EXCEPTION_MONITOR_0,
    core_0_iram0_exception_monitor_1: CORE_0_IRAM0_EXCEPTION_MONITOR_1,
    core_0_dram0_exception_monitor_0: CORE_0_DRAM0_EXCEPTION_MONITOR_0,
    core_0_dram0_exception_monitor_1: CORE_0_DRAM0_EXCEPTION_MONITOR_1,
    core_0_dram0_exception_monitor_2: CORE_0_DRAM0_EXCEPTION_MONITOR_2,
    core_0_dram0_exception_monitor_3: CORE_0_DRAM0_EXCEPTION_MONITOR_3,
    core_x_iram0_dram0_exception_monitor_0: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0,
    core_x_iram0_dram0_exception_monitor_1: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1,
    log_setting: LOG_SETTING,
    log_data_0: LOG_DATA_0,
    log_data_mask: LOG_DATA_MASK,
    log_min: LOG_MIN,
    log_max: LOG_MAX,
    log_mem_start: LOG_MEM_START,
    log_mem_end: LOG_MEM_END,
    log_mem_writing_addr: LOG_MEM_WRITING_ADDR,
    log_mem_full_flag: LOG_MEM_FULL_FLAG,
    c0re_0_lastpc_before_exception: C0RE_0_LASTPC_BEFORE_EXCEPTION,
    c0re_0_debug_mode: C0RE_0_DEBUG_MODE,
    _reserved39: [u8; 0x0160],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - ASSIST_DEBUG_C0RE_0_MONTR_ENA_REG"]
    #[inline(always)]
    pub const fn core_0_montr_ena(&self) -> &CORE_0_MONTR_ENA {
        &self.core_0_montr_ena
    }
    #[doc = "0x04 - ASSIST_DEBUG_CORE_0_INTR_RAW_REG"]
    #[inline(always)]
    pub const fn core_0_intr_raw(&self) -> &CORE_0_INTR_RAW {
        &self.core_0_intr_raw
    }
    #[doc = "0x08 - ASSIST_DEBUG_CORE_0_INTR_ENA_REG"]
    #[inline(always)]
    pub const fn core_0_intr_ena(&self) -> &CORE_0_INTR_ENA {
        &self.core_0_intr_ena
    }
    #[doc = "0x0c - ASSIST_DEBUG_CORE_0_INTR_CLR_REG"]
    #[inline(always)]
    pub const fn core_0_intr_clr(&self) -> &CORE_0_INTR_CLR {
        &self.core_0_intr_clr
    }
    #[doc = "0x10 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN_REG"]
    #[inline(always)]
    pub const fn core_0_area_dram0_0_min(&self) -> &CORE_0_AREA_DRAM0_0_MIN {
        &self.core_0_area_dram0_0_min
    }
    #[doc = "0x14 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_REG"]
    #[inline(always)]
    pub const fn core_0_area_dram0_0_max(&self) -> &CORE_0_AREA_DRAM0_0_MAX {
        &self.core_0_area_dram0_0_max
    }
    #[doc = "0x18 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN_REG"]
    #[inline(always)]
    pub const fn core_0_area_dram0_1_min(&self) -> &CORE_0_AREA_DRAM0_1_MIN {
        &self.core_0_area_dram0_1_min
    }
    #[doc = "0x1c - ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX_REG"]
    #[inline(always)]
    pub const fn core_0_area_dram0_1_max(&self) -> &CORE_0_AREA_DRAM0_1_MAX {
        &self.core_0_area_dram0_1_max
    }
    #[doc = "0x20 - ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN_REG"]
    #[inline(always)]
    pub const fn core_0_area_pif_0_min(&self) -> &CORE_0_AREA_PIF_0_MIN {
        &self.core_0_area_pif_0_min
    }
    #[doc = "0x24 - ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX_REG"]
    #[inline(always)]
    pub const fn core_0_area_pif_0_max(&self) -> &CORE_0_AREA_PIF_0_MAX {
        &self.core_0_area_pif_0_max
    }
    #[doc = "0x28 - ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN_REG"]
    #[inline(always)]
    pub const fn core_0_area_pif_1_min(&self) -> &CORE_0_AREA_PIF_1_MIN {
        &self.core_0_area_pif_1_min
    }
    #[doc = "0x2c - ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX_REG"]
    #[inline(always)]
    pub const fn core_0_area_pif_1_max(&self) -> &CORE_0_AREA_PIF_1_MAX {
        &self.core_0_area_pif_1_max
    }
    #[doc = "0x30 - ASSIST_DEBUG_CORE_0_AREA_PC_REG"]
    #[inline(always)]
    pub const fn core_0_area_pc(&self) -> &CORE_0_AREA_PC {
        &self.core_0_area_pc
    }
    #[doc = "0x34 - ASSIST_DEBUG_CORE_0_AREA_SP_REG"]
    #[inline(always)]
    pub const fn core_0_area_sp(&self) -> &CORE_0_AREA_SP {
        &self.core_0_area_sp
    }
    #[doc = "0x38 - ASSIST_DEBUG_CORE_0_SP_MIN_REG"]
    #[inline(always)]
    pub const fn core_0_sp_min(&self) -> &CORE_0_SP_MIN {
        &self.core_0_sp_min
    }
    #[doc = "0x3c - ASSIST_DEBUG_CORE_0_SP_MAX_REG"]
    #[inline(always)]
    pub const fn core_0_sp_max(&self) -> &CORE_0_SP_MAX {
        &self.core_0_sp_max
    }
    #[doc = "0x40 - ASSIST_DEBUG_CORE_0_SP_PC_REG"]
    #[inline(always)]
    pub const fn core_0_sp_pc(&self) -> &CORE_0_SP_PC {
        &self.core_0_sp_pc
    }
    #[doc = "0x44 - ASSIST_DEBUG_CORE_0_RCD_EN_REG"]
    #[inline(always)]
    pub const fn core_0_rcd_en(&self) -> &CORE_0_RCD_EN {
        &self.core_0_rcd_en
    }
    #[doc = "0x48 - ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugpc(&self) -> &CORE_0_RCD_PDEBUGPC {
        &self.core_0_rcd_pdebugpc
    }
    #[doc = "0x4c - ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugsp(&self) -> &CORE_0_RCD_PDEBUGSP {
        &self.core_0_rcd_pdebugsp
    }
    #[doc = "0x50 - ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
    #[inline(always)]
    pub const fn core_0_iram0_exception_monitor_0(&self) -> &CORE_0_IRAM0_EXCEPTION_MONITOR_0 {
        &self.core_0_iram0_exception_monitor_0
    }
    #[doc = "0x54 - ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1_REG"]
    #[inline(always)]
    pub const fn core_0_iram0_exception_monitor_1(&self) -> &CORE_0_IRAM0_EXCEPTION_MONITOR_1 {
        &self.core_0_iram0_exception_monitor_1
    }
    #[doc = "0x58 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0_REG"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_0(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_0 {
        &self.core_0_dram0_exception_monitor_0
    }
    #[doc = "0x5c - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_1(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_1 {
        &self.core_0_dram0_exception_monitor_1
    }
    #[doc = "0x60 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_2(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_2 {
        &self.core_0_dram0_exception_monitor_2
    }
    #[doc = "0x64 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3_REG"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_3(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_3 {
        &self.core_0_dram0_exception_monitor_3
    }
    #[doc = "0x68 - ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_REG"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor_0(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 {
        &self.core_x_iram0_dram0_exception_monitor_0
    }
    #[doc = "0x6c - ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_REG"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor_1(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 {
        &self.core_x_iram0_dram0_exception_monitor_1
    }
    #[doc = "0x70 - ASSIST_DEBUG_LOG_SETTING"]
    #[inline(always)]
    pub const fn log_setting(&self) -> &LOG_SETTING {
        &self.log_setting
    }
    #[doc = "0x74 - ASSIST_DEBUG_LOG_DATA_0_REG"]
    #[inline(always)]
    pub const fn log_data_0(&self) -> &LOG_DATA_0 {
        &self.log_data_0
    }
    #[doc = "0x78 - ASSIST_DEBUG_LOG_DATA_MASK_REG"]
    #[inline(always)]
    pub const fn log_data_mask(&self) -> &LOG_DATA_MASK {
        &self.log_data_mask
    }
    #[doc = "0x7c - ASSIST_DEBUG_LOG_MIN_REG"]
    #[inline(always)]
    pub const fn log_min(&self) -> &LOG_MIN {
        &self.log_min
    }
    #[doc = "0x80 - ASSIST_DEBUG_LOG_MAX_REG"]
    #[inline(always)]
    pub const fn log_max(&self) -> &LOG_MAX {
        &self.log_max
    }
    #[doc = "0x84 - ASSIST_DEBUG_LOG_MEM_START_REG"]
    #[inline(always)]
    pub const fn log_mem_start(&self) -> &LOG_MEM_START {
        &self.log_mem_start
    }
    #[doc = "0x88 - ASSIST_DEBUG_LOG_MEM_END_REG"]
    #[inline(always)]
    pub const fn log_mem_end(&self) -> &LOG_MEM_END {
        &self.log_mem_end
    }
    #[doc = "0x8c - ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_REG"]
    #[inline(always)]
    pub const fn log_mem_writing_addr(&self) -> &LOG_MEM_WRITING_ADDR {
        &self.log_mem_writing_addr
    }
    #[doc = "0x90 - ASSIST_DEBUG_LOG_MEM_FULL_FLAG_REG"]
    #[inline(always)]
    pub const fn log_mem_full_flag(&self) -> &LOG_MEM_FULL_FLAG {
        &self.log_mem_full_flag
    }
    #[doc = "0x94 - ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
    #[inline(always)]
    pub const fn c0re_0_lastpc_before_exception(&self) -> &C0RE_0_LASTPC_BEFORE_EXCEPTION {
        &self.c0re_0_lastpc_before_exception
    }
    #[doc = "0x98 - ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
    #[inline(always)]
    pub const fn c0re_0_debug_mode(&self) -> &C0RE_0_DEBUG_MODE {
        &self.c0re_0_debug_mode
    }
    #[doc = "0x1fc - ASSIST_DEBUG_DATE_REG"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CORE_0_MONTR_ENA (rw) register accessor: ASSIST_DEBUG_C0RE_0_MONTR_ENA_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_montr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_montr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_montr_ena`] module"]
pub type CORE_0_MONTR_ENA = crate::Reg<core_0_montr_ena::CORE_0_MONTR_ENA_SPEC>;
#[doc = "ASSIST_DEBUG_C0RE_0_MONTR_ENA_REG"]
pub mod core_0_montr_ena;
#[doc = "CORE_0_INTR_RAW (r) register accessor: ASSIST_DEBUG_CORE_0_INTR_RAW_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_intr_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_raw`] module"]
pub type CORE_0_INTR_RAW = crate::Reg<core_0_intr_raw::CORE_0_INTR_RAW_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RAW_REG"]
pub mod core_0_intr_raw;
#[doc = "CORE_0_INTR_ENA (rw) register accessor: ASSIST_DEBUG_CORE_0_INTR_ENA_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_intr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_intr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_ena`] module"]
pub type CORE_0_INTR_ENA = crate::Reg<core_0_intr_ena::CORE_0_INTR_ENA_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_ENA_REG"]
pub mod core_0_intr_ena;
#[doc = "CORE_0_INTR_CLR (rw) register accessor: ASSIST_DEBUG_CORE_0_INTR_CLR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_intr_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_intr_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_clr`] module"]
pub type CORE_0_INTR_CLR = crate::Reg<core_0_intr_clr::CORE_0_INTR_CLR_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_CLR_REG"]
pub mod core_0_intr_clr;
#[doc = "CORE_0_AREA_DRAM0_0_MIN (rw) register accessor: ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_dram0_0_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_dram0_0_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_0_min`] module"]
pub type CORE_0_AREA_DRAM0_0_MIN =
    crate::Reg<core_0_area_dram0_0_min::CORE_0_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN_REG"]
pub mod core_0_area_dram0_0_min;
#[doc = "CORE_0_AREA_DRAM0_0_MAX (rw) register accessor: ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_dram0_0_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_dram0_0_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_0_max`] module"]
pub type CORE_0_AREA_DRAM0_0_MAX =
    crate::Reg<core_0_area_dram0_0_max::CORE_0_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_REG"]
pub mod core_0_area_dram0_0_max;
#[doc = "CORE_0_AREA_DRAM0_1_MIN (rw) register accessor: ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_dram0_1_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_dram0_1_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_1_min`] module"]
pub type CORE_0_AREA_DRAM0_1_MIN =
    crate::Reg<core_0_area_dram0_1_min::CORE_0_AREA_DRAM0_1_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN_REG"]
pub mod core_0_area_dram0_1_min;
#[doc = "CORE_0_AREA_DRAM0_1_MAX (rw) register accessor: ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_dram0_1_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_dram0_1_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_1_max`] module"]
pub type CORE_0_AREA_DRAM0_1_MAX =
    crate::Reg<core_0_area_dram0_1_max::CORE_0_AREA_DRAM0_1_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX_REG"]
pub mod core_0_area_dram0_1_max;
#[doc = "CORE_0_AREA_PIF_0_MIN (rw) register accessor: ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pif_0_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_pif_0_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_0_min`] module"]
pub type CORE_0_AREA_PIF_0_MIN = crate::Reg<core_0_area_pif_0_min::CORE_0_AREA_PIF_0_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN_REG"]
pub mod core_0_area_pif_0_min;
#[doc = "CORE_0_AREA_PIF_0_MAX (rw) register accessor: ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pif_0_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_pif_0_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_0_max`] module"]
pub type CORE_0_AREA_PIF_0_MAX = crate::Reg<core_0_area_pif_0_max::CORE_0_AREA_PIF_0_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX_REG"]
pub mod core_0_area_pif_0_max;
#[doc = "CORE_0_AREA_PIF_1_MIN (rw) register accessor: ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pif_1_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_pif_1_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_1_min`] module"]
pub type CORE_0_AREA_PIF_1_MIN = crate::Reg<core_0_area_pif_1_min::CORE_0_AREA_PIF_1_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN_REG"]
pub mod core_0_area_pif_1_min;
#[doc = "CORE_0_AREA_PIF_1_MAX (rw) register accessor: ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pif_1_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_pif_1_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_1_max`] module"]
pub type CORE_0_AREA_PIF_1_MAX = crate::Reg<core_0_area_pif_1_max::CORE_0_AREA_PIF_1_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX_REG"]
pub mod core_0_area_pif_1_max;
#[doc = "CORE_0_AREA_PC (r) register accessor: ASSIST_DEBUG_CORE_0_AREA_PC_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pc`] module"]
pub type CORE_0_AREA_PC = crate::Reg<core_0_area_pc::CORE_0_AREA_PC_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PC_REG"]
pub mod core_0_area_pc;
#[doc = "CORE_0_AREA_SP (r) register accessor: ASSIST_DEBUG_CORE_0_AREA_SP_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_sp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_sp`] module"]
pub type CORE_0_AREA_SP = crate::Reg<core_0_area_sp::CORE_0_AREA_SP_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_SP_REG"]
pub mod core_0_area_sp;
#[doc = "CORE_0_SP_MIN (rw) register accessor: ASSIST_DEBUG_CORE_0_SP_MIN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_sp_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_min`] module"]
pub type CORE_0_SP_MIN = crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_SP_MIN_REG"]
pub mod core_0_sp_min;
#[doc = "CORE_0_SP_MAX (rw) register accessor: ASSIST_DEBUG_CORE_0_SP_MAX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_sp_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_max`] module"]
pub type CORE_0_SP_MAX = crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_SP_MAX_REG"]
pub mod core_0_sp_max;
#[doc = "CORE_0_SP_PC (r) register accessor: ASSIST_DEBUG_CORE_0_SP_PC_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_pc`] module"]
pub type CORE_0_SP_PC = crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_SP_PC_REG"]
pub mod core_0_sp_pc;
#[doc = "CORE_0_RCD_EN (rw) register accessor: ASSIST_DEBUG_CORE_0_RCD_EN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_rcd_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_en`] module"]
pub type CORE_0_RCD_EN = crate::Reg<core_0_rcd_en::CORE_0_RCD_EN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_EN_REG"]
pub mod core_0_rcd_en;
#[doc = "CORE_0_RCD_PDEBUGPC (r) register accessor: ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugpc`] module"]
pub type CORE_0_RCD_PDEBUGPC = crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG"]
pub mod core_0_rcd_pdebugpc;
#[doc = "CORE_0_RCD_PDEBUGSP (r) register accessor: ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugsp`] module"]
pub type CORE_0_RCD_PDEBUGSP = crate::Reg<core_0_rcd_pdebugsp::CORE_0_RCD_PDEBUGSP_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
pub mod core_0_rcd_pdebugsp;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_0 (r) register accessor: ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_exception_monitor_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_exception_monitor_0`] module"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_iram0_exception_monitor_0::CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
pub mod core_0_iram0_exception_monitor_0;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_1 (r) register accessor: ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_exception_monitor_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_exception_monitor_1`] module"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_iram0_exception_monitor_1::CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod core_0_iram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_0 (r) register accessor: ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_0`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_dram0_exception_monitor_0::CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0_REG"]
pub mod core_0_dram0_exception_monitor_0;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_1 (r) register accessor: ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_1`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_dram0_exception_monitor_1::CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod core_0_dram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_2 (r) register accessor: ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_2`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<core_0_dram0_exception_monitor_2::CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod core_0_dram0_exception_monitor_2;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_3 (r) register accessor: ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_3`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<core_0_dram0_exception_monitor_3::CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3_REG"]
pub mod core_0_dram0_exception_monitor_3;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 (rw) register accessor: ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor_0`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_REG"]
pub mod core_x_iram0_dram0_exception_monitor_0;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 (rw) register accessor: ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor_1`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_1::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod core_x_iram0_dram0_exception_monitor_1;
#[doc = "LOG_SETTING (rw) register accessor: ASSIST_DEBUG_LOG_SETTING\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_setting::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_setting::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_setting`] module"]
pub type LOG_SETTING = crate::Reg<log_setting::LOG_SETTING_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_SETTING"]
pub mod log_setting;
#[doc = "LOG_DATA_0 (rw) register accessor: ASSIST_DEBUG_LOG_DATA_0_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_0`] module"]
pub type LOG_DATA_0 = crate::Reg<log_data_0::LOG_DATA_0_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_DATA_0_REG"]
pub mod log_data_0;
#[doc = "LOG_DATA_MASK (rw) register accessor: ASSIST_DEBUG_LOG_DATA_MASK_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_mask`] module"]
pub type LOG_DATA_MASK = crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_DATA_MASK_REG"]
pub mod log_data_mask;
#[doc = "LOG_MIN (rw) register accessor: ASSIST_DEBUG_LOG_MIN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_min`] module"]
pub type LOG_MIN = crate::Reg<log_min::LOG_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MIN_REG"]
pub mod log_min;
#[doc = "LOG_MAX (rw) register accessor: ASSIST_DEBUG_LOG_MAX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_max`] module"]
pub type LOG_MAX = crate::Reg<log_max::LOG_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MAX_REG"]
pub mod log_max;
#[doc = "LOG_MEM_START (rw) register accessor: ASSIST_DEBUG_LOG_MEM_START_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_mem_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_mem_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_start`] module"]
pub type LOG_MEM_START = crate::Reg<log_mem_start::LOG_MEM_START_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_START_REG"]
pub mod log_mem_start;
#[doc = "LOG_MEM_END (rw) register accessor: ASSIST_DEBUG_LOG_MEM_END_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_mem_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_mem_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_end`] module"]
pub type LOG_MEM_END = crate::Reg<log_mem_end::LOG_MEM_END_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_END_REG"]
pub mod log_mem_end;
#[doc = "LOG_MEM_WRITING_ADDR (r) register accessor: ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_mem_writing_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_writing_addr`] module"]
pub type LOG_MEM_WRITING_ADDR = crate::Reg<log_mem_writing_addr::LOG_MEM_WRITING_ADDR_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_REG"]
pub mod log_mem_writing_addr;
#[doc = "LOG_MEM_FULL_FLAG (rw) register accessor: ASSIST_DEBUG_LOG_MEM_FULL_FLAG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_mem_full_flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_mem_full_flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_full_flag`] module"]
pub type LOG_MEM_FULL_FLAG = crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_FULL_FLAG_REG"]
pub mod log_mem_full_flag;
#[doc = "C0RE_0_LASTPC_BEFORE_EXCEPTION (r) register accessor: ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0re_0_lastpc_before_exception::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0re_0_lastpc_before_exception`] module"]
pub type C0RE_0_LASTPC_BEFORE_EXCEPTION =
    crate::Reg<c0re_0_lastpc_before_exception::C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
pub mod c0re_0_lastpc_before_exception;
#[doc = "C0RE_0_DEBUG_MODE (r) register accessor: ASSIST_DEBUG_C0RE_0_DEBUG_MODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0re_0_debug_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0re_0_debug_mode`] module"]
pub type C0RE_0_DEBUG_MODE = crate::Reg<c0re_0_debug_mode::C0RE_0_DEBUG_MODE_SPEC>;
#[doc = "ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
pub mod c0re_0_debug_mode;
#[doc = "DATE (rw) register accessor: ASSIST_DEBUG_DATE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "ASSIST_DEBUG_DATE_REG"]
pub mod date;
