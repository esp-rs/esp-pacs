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
    core_0_dram0_exception_monitor_4: CORE_0_DRAM0_EXCEPTION_MONITOR_4,
    core_0_dram0_exception_monitor_5: CORE_0_DRAM0_EXCEPTION_MONITOR_5,
    core_0_lastpc_before_exception: CORE_0_LASTPC_BEFORE_EXCEPTION,
    core_0_debug_mode: CORE_0_DEBUG_MODE,
    _reserved30: [u8; 0x88],
    core_x_iram0_dram0_exception_monitor_0: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0,
    core_x_iram0_dram0_exception_monitor_1: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1,
    clock_gate: CLOCK_GATE,
    _reserved33: [u8; 0x02f0],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - core0 monitor enable configuration register"]
    #[inline(always)]
    pub const fn core_0_montr_ena(&self) -> &CORE_0_MONTR_ENA {
        &self.core_0_montr_ena
    }
    #[doc = "0x04 - core0 monitor interrupt status register"]
    #[inline(always)]
    pub const fn core_0_intr_raw(&self) -> &CORE_0_INTR_RAW {
        &self.core_0_intr_raw
    }
    #[doc = "0x08 - core0 monitor interrupt enable register"]
    #[inline(always)]
    pub const fn core_0_intr_ena(&self) -> &CORE_0_INTR_ENA {
        &self.core_0_intr_ena
    }
    #[doc = "0x0c - core0 monitor interrupt clear register"]
    #[inline(always)]
    pub const fn core_0_intr_clr(&self) -> &CORE_0_INTR_CLR {
        &self.core_0_intr_clr
    }
    #[doc = "0x10 - Configures lower boundary address of region 0 monitored on Data bus"]
    #[inline(always)]
    pub const fn core_0_area_dram0_0_min(&self) -> &CORE_0_AREA_DRAM0_0_MIN {
        &self.core_0_area_dram0_0_min
    }
    #[doc = "0x14 - Configures upper boundary address of region 0 monitored on Data bus"]
    #[inline(always)]
    pub const fn core_0_area_dram0_0_max(&self) -> &CORE_0_AREA_DRAM0_0_MAX {
        &self.core_0_area_dram0_0_max
    }
    #[doc = "0x18 - Configures lower boundary address of region 1 monitored on Data bus"]
    #[inline(always)]
    pub const fn core_0_area_dram0_1_min(&self) -> &CORE_0_AREA_DRAM0_1_MIN {
        &self.core_0_area_dram0_1_min
    }
    #[doc = "0x1c - Configures upper boundary address of region 1 monitored on Data bus"]
    #[inline(always)]
    pub const fn core_0_area_dram0_1_max(&self) -> &CORE_0_AREA_DRAM0_1_MAX {
        &self.core_0_area_dram0_1_max
    }
    #[doc = "0x20 - Configures lower boundary address of region 0 monitored on Peripheral bus"]
    #[inline(always)]
    pub const fn core_0_area_pif_0_min(&self) -> &CORE_0_AREA_PIF_0_MIN {
        &self.core_0_area_pif_0_min
    }
    #[doc = "0x24 - Configures upper boundary address of region 0 monitored on Peripheral bus"]
    #[inline(always)]
    pub const fn core_0_area_pif_0_max(&self) -> &CORE_0_AREA_PIF_0_MAX {
        &self.core_0_area_pif_0_max
    }
    #[doc = "0x28 - Configures lower boundary address of region 1 monitored on Peripheral bus"]
    #[inline(always)]
    pub const fn core_0_area_pif_1_min(&self) -> &CORE_0_AREA_PIF_1_MIN {
        &self.core_0_area_pif_1_min
    }
    #[doc = "0x2c - Configures upper boundary address of region 1 monitored on Peripheral bus"]
    #[inline(always)]
    pub const fn core_0_area_pif_1_max(&self) -> &CORE_0_AREA_PIF_1_MAX {
        &self.core_0_area_pif_1_max
    }
    #[doc = "0x30 - Region monitoring HP CPU PC status register"]
    #[inline(always)]
    pub const fn core_0_area_pc(&self) -> &CORE_0_AREA_PC {
        &self.core_0_area_pc
    }
    #[doc = "0x34 - Region monitoring HP CPU SP status register"]
    #[inline(always)]
    pub const fn core_0_area_sp(&self) -> &CORE_0_AREA_SP {
        &self.core_0_area_sp
    }
    #[doc = "0x38 - Configures stack monitoring lower boundary address"]
    #[inline(always)]
    pub const fn core_0_sp_min(&self) -> &CORE_0_SP_MIN {
        &self.core_0_sp_min
    }
    #[doc = "0x3c - Configures stack monitoring upper boundary address"]
    #[inline(always)]
    pub const fn core_0_sp_max(&self) -> &CORE_0_SP_MAX {
        &self.core_0_sp_max
    }
    #[doc = "0x40 - Stack monitoring HP CPU PC status register"]
    #[inline(always)]
    pub const fn core_0_sp_pc(&self) -> &CORE_0_SP_PC {
        &self.core_0_sp_pc
    }
    #[doc = "0x44 - HP CPU PC logging enable register"]
    #[inline(always)]
    pub const fn core_0_rcd_en(&self) -> &CORE_0_RCD_EN {
        &self.core_0_rcd_en
    }
    #[doc = "0x48 - PC logging register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugpc(&self) -> &CORE_0_RCD_PDEBUGPC {
        &self.core_0_rcd_pdebugpc
    }
    #[doc = "0x4c - PC logging register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugsp(&self) -> &CORE_0_RCD_PDEBUGSP {
        &self.core_0_rcd_pdebugsp
    }
    #[doc = "0x50 - exception monitor status register0"]
    #[inline(always)]
    pub const fn core_0_iram0_exception_monitor_0(&self) -> &CORE_0_IRAM0_EXCEPTION_MONITOR_0 {
        &self.core_0_iram0_exception_monitor_0
    }
    #[doc = "0x54 - exception monitor status register1"]
    #[inline(always)]
    pub const fn core_0_iram0_exception_monitor_1(&self) -> &CORE_0_IRAM0_EXCEPTION_MONITOR_1 {
        &self.core_0_iram0_exception_monitor_1
    }
    #[doc = "0x58 - exception monitor status register2"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_0(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_0 {
        &self.core_0_dram0_exception_monitor_0
    }
    #[doc = "0x5c - exception monitor status register3"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_1(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_1 {
        &self.core_0_dram0_exception_monitor_1
    }
    #[doc = "0x60 - exception monitor status register4"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_2(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_2 {
        &self.core_0_dram0_exception_monitor_2
    }
    #[doc = "0x64 - exception monitor status register5"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_3(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_3 {
        &self.core_0_dram0_exception_monitor_3
    }
    #[doc = "0x68 - exception monitor status register6"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_4(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_4 {
        &self.core_0_dram0_exception_monitor_4
    }
    #[doc = "0x6c - exception monitor status register7"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_5(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_5 {
        &self.core_0_dram0_exception_monitor_5
    }
    #[doc = "0x70 - cpu status register"]
    #[inline(always)]
    pub const fn core_0_lastpc_before_exception(&self) -> &CORE_0_LASTPC_BEFORE_EXCEPTION {
        &self.core_0_lastpc_before_exception
    }
    #[doc = "0x74 - cpu status register"]
    #[inline(always)]
    pub const fn core_0_debug_mode(&self) -> &CORE_0_DEBUG_MODE {
        &self.core_0_debug_mode
    }
    #[doc = "0x100 - exception monitor status register8"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor_0(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 {
        &self.core_x_iram0_dram0_exception_monitor_0
    }
    #[doc = "0x104 - exception monitor status register9"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor_1(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 {
        &self.core_x_iram0_dram0_exception_monitor_1
    }
    #[doc = "0x108 - Register clock control"]
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
#[doc = "CORE_0_MONTR_ENA (rw) register accessor: core0 monitor enable configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_montr_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_montr_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_montr_ena`] module"]
pub type CORE_0_MONTR_ENA = crate::Reg<core_0_montr_ena::CORE_0_MONTR_ENA_SPEC>;
#[doc = "core0 monitor enable configuration register"]
pub mod core_0_montr_ena;
#[doc = "CORE_0_INTR_RAW (r) register accessor: core0 monitor interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_raw`] module"]
pub type CORE_0_INTR_RAW = crate::Reg<core_0_intr_raw::CORE_0_INTR_RAW_SPEC>;
#[doc = "core0 monitor interrupt status register"]
pub mod core_0_intr_raw;
#[doc = "CORE_0_INTR_ENA (rw) register accessor: core0 monitor interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_ena`] module"]
pub type CORE_0_INTR_ENA = crate::Reg<core_0_intr_ena::CORE_0_INTR_ENA_SPEC>;
#[doc = "core0 monitor interrupt enable register"]
pub mod core_0_intr_ena;
#[doc = "CORE_0_INTR_CLR (w) register accessor: core0 monitor interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_clr`] module"]
pub type CORE_0_INTR_CLR = crate::Reg<core_0_intr_clr::CORE_0_INTR_CLR_SPEC>;
#[doc = "core0 monitor interrupt clear register"]
pub mod core_0_intr_clr;
#[doc = "CORE_0_AREA_DRAM0_0_MIN (rw) register accessor: Configures lower boundary address of region 0 monitored on Data bus\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_dram0_0_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_dram0_0_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_0_min`] module"]
pub type CORE_0_AREA_DRAM0_0_MIN =
    crate::Reg<core_0_area_dram0_0_min::CORE_0_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "Configures lower boundary address of region 0 monitored on Data bus"]
pub mod core_0_area_dram0_0_min;
#[doc = "CORE_0_AREA_DRAM0_0_MAX (rw) register accessor: Configures upper boundary address of region 0 monitored on Data bus\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_dram0_0_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_dram0_0_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_0_max`] module"]
pub type CORE_0_AREA_DRAM0_0_MAX =
    crate::Reg<core_0_area_dram0_0_max::CORE_0_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "Configures upper boundary address of region 0 monitored on Data bus"]
pub mod core_0_area_dram0_0_max;
#[doc = "CORE_0_AREA_DRAM0_1_MIN (rw) register accessor: Configures lower boundary address of region 1 monitored on Data bus\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_dram0_1_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_dram0_1_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_1_min`] module"]
pub type CORE_0_AREA_DRAM0_1_MIN =
    crate::Reg<core_0_area_dram0_1_min::CORE_0_AREA_DRAM0_1_MIN_SPEC>;
#[doc = "Configures lower boundary address of region 1 monitored on Data bus"]
pub mod core_0_area_dram0_1_min;
#[doc = "CORE_0_AREA_DRAM0_1_MAX (rw) register accessor: Configures upper boundary address of region 1 monitored on Data bus\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_dram0_1_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_dram0_1_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_1_max`] module"]
pub type CORE_0_AREA_DRAM0_1_MAX =
    crate::Reg<core_0_area_dram0_1_max::CORE_0_AREA_DRAM0_1_MAX_SPEC>;
#[doc = "Configures upper boundary address of region 1 monitored on Data bus"]
pub mod core_0_area_dram0_1_max;
#[doc = "CORE_0_AREA_PIF_0_MIN (rw) register accessor: Configures lower boundary address of region 0 monitored on Peripheral bus\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_pif_0_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_pif_0_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_0_min`] module"]
pub type CORE_0_AREA_PIF_0_MIN = crate::Reg<core_0_area_pif_0_min::CORE_0_AREA_PIF_0_MIN_SPEC>;
#[doc = "Configures lower boundary address of region 0 monitored on Peripheral bus"]
pub mod core_0_area_pif_0_min;
#[doc = "CORE_0_AREA_PIF_0_MAX (rw) register accessor: Configures upper boundary address of region 0 monitored on Peripheral bus\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_pif_0_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_pif_0_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_0_max`] module"]
pub type CORE_0_AREA_PIF_0_MAX = crate::Reg<core_0_area_pif_0_max::CORE_0_AREA_PIF_0_MAX_SPEC>;
#[doc = "Configures upper boundary address of region 0 monitored on Peripheral bus"]
pub mod core_0_area_pif_0_max;
#[doc = "CORE_0_AREA_PIF_1_MIN (rw) register accessor: Configures lower boundary address of region 1 monitored on Peripheral bus\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_pif_1_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_pif_1_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_1_min`] module"]
pub type CORE_0_AREA_PIF_1_MIN = crate::Reg<core_0_area_pif_1_min::CORE_0_AREA_PIF_1_MIN_SPEC>;
#[doc = "Configures lower boundary address of region 1 monitored on Peripheral bus"]
pub mod core_0_area_pif_1_min;
#[doc = "CORE_0_AREA_PIF_1_MAX (rw) register accessor: Configures upper boundary address of region 1 monitored on Peripheral bus\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_pif_1_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_pif_1_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_1_max`] module"]
pub type CORE_0_AREA_PIF_1_MAX = crate::Reg<core_0_area_pif_1_max::CORE_0_AREA_PIF_1_MAX_SPEC>;
#[doc = "Configures upper boundary address of region 1 monitored on Peripheral bus"]
pub mod core_0_area_pif_1_max;
#[doc = "CORE_0_AREA_PC (r) register accessor: Region monitoring HP CPU PC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pc`] module"]
pub type CORE_0_AREA_PC = crate::Reg<core_0_area_pc::CORE_0_AREA_PC_SPEC>;
#[doc = "Region monitoring HP CPU PC status register"]
pub mod core_0_area_pc;
#[doc = "CORE_0_AREA_SP (r) register accessor: Region monitoring HP CPU SP status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_sp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_sp`] module"]
pub type CORE_0_AREA_SP = crate::Reg<core_0_area_sp::CORE_0_AREA_SP_SPEC>;
#[doc = "Region monitoring HP CPU SP status register"]
pub mod core_0_area_sp;
#[doc = "CORE_0_SP_MIN (rw) register accessor: Configures stack monitoring lower boundary address\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_sp_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_sp_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_min`] module"]
pub type CORE_0_SP_MIN = crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>;
#[doc = "Configures stack monitoring lower boundary address"]
pub mod core_0_sp_min;
#[doc = "CORE_0_SP_MAX (rw) register accessor: Configures stack monitoring upper boundary address\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_sp_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_sp_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_max`] module"]
pub type CORE_0_SP_MAX = crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>;
#[doc = "Configures stack monitoring upper boundary address"]
pub mod core_0_sp_max;
#[doc = "CORE_0_SP_PC (r) register accessor: Stack monitoring HP CPU PC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_sp_pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_pc`] module"]
pub type CORE_0_SP_PC = crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>;
#[doc = "Stack monitoring HP CPU PC status register"]
pub mod core_0_sp_pc;
#[doc = "CORE_0_RCD_EN (rw) register accessor: HP CPU PC logging enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_rcd_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_rcd_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_en`] module"]
pub type CORE_0_RCD_EN = crate::Reg<core_0_rcd_en::CORE_0_RCD_EN_SPEC>;
#[doc = "HP CPU PC logging enable register"]
pub mod core_0_rcd_en;
#[doc = "CORE_0_RCD_PDEBUGPC (r) register accessor: PC logging register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_rcd_pdebugpc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugpc`] module"]
pub type CORE_0_RCD_PDEBUGPC = crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>;
#[doc = "PC logging register"]
pub mod core_0_rcd_pdebugpc;
#[doc = "CORE_0_RCD_PDEBUGSP (r) register accessor: PC logging register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_rcd_pdebugsp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugsp`] module"]
pub type CORE_0_RCD_PDEBUGSP = crate::Reg<core_0_rcd_pdebugsp::CORE_0_RCD_PDEBUGSP_SPEC>;
#[doc = "PC logging register"]
pub mod core_0_rcd_pdebugsp;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_0 (r) register accessor: exception monitor status register0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_iram0_exception_monitor_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_exception_monitor_0`] module"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_iram0_exception_monitor_0::CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "exception monitor status register0"]
pub mod core_0_iram0_exception_monitor_0;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_1 (r) register accessor: exception monitor status register1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_iram0_exception_monitor_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_exception_monitor_1`] module"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_iram0_exception_monitor_1::CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "exception monitor status register1"]
pub mod core_0_iram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_0 (r) register accessor: exception monitor status register2\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_dram0_exception_monitor_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_0`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_dram0_exception_monitor_0::CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "exception monitor status register2"]
pub mod core_0_dram0_exception_monitor_0;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_1 (r) register accessor: exception monitor status register3\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_dram0_exception_monitor_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_1`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_dram0_exception_monitor_1::CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "exception monitor status register3"]
pub mod core_0_dram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_2 (r) register accessor: exception monitor status register4\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_dram0_exception_monitor_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_2`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<core_0_dram0_exception_monitor_2::CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "exception monitor status register4"]
pub mod core_0_dram0_exception_monitor_2;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_3 (r) register accessor: exception monitor status register5\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_dram0_exception_monitor_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_3`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<core_0_dram0_exception_monitor_3::CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "exception monitor status register5"]
pub mod core_0_dram0_exception_monitor_3;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_4 (r) register accessor: exception monitor status register6\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_dram0_exception_monitor_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_4`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_4 =
    crate::Reg<core_0_dram0_exception_monitor_4::CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC>;
#[doc = "exception monitor status register6"]
pub mod core_0_dram0_exception_monitor_4;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_5 (r) register accessor: exception monitor status register7\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_dram0_exception_monitor_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_5`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_5 =
    crate::Reg<core_0_dram0_exception_monitor_5::CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>;
#[doc = "exception monitor status register7"]
pub mod core_0_dram0_exception_monitor_5;
#[doc = "CORE_0_LASTPC_BEFORE_EXCEPTION (r) register accessor: cpu status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_lastpc_before_exception::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_lastpc_before_exception`] module"]
pub type CORE_0_LASTPC_BEFORE_EXCEPTION =
    crate::Reg<core_0_lastpc_before_exception::CORE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "cpu status register"]
pub mod core_0_lastpc_before_exception;
#[doc = "CORE_0_DEBUG_MODE (r) register accessor: cpu status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_debug_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_debug_mode`] module"]
pub type CORE_0_DEBUG_MODE = crate::Reg<core_0_debug_mode::CORE_0_DEBUG_MODE_SPEC>;
#[doc = "cpu status register"]
pub mod core_0_debug_mode;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 (rw) register accessor: exception monitor status register8\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor_0`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "exception monitor status register8"]
pub mod core_x_iram0_dram0_exception_monitor_0;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 (rw) register accessor: exception monitor status register9\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor_1`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_1::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "exception monitor status register9"]
pub mod core_x_iram0_dram0_exception_monitor_1;
#[doc = "CLOCK_GATE (rw) register accessor: Register clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Register clock control"]
pub mod clock_gate;
pub use crate::aes::date;
pub use crate::aes::DATE;
