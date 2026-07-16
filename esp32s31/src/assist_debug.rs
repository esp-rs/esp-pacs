#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu: [CPU; 2],
    core_x_iram0_dram0_exception_monitor_0: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0,
    clock_gate: CLOCK_GATE,
    _reserved3: [u8; 0x02e4],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x110 - Cluster CPU%s, containing CORE_?_MONTR_ENA, CORE_?_INTR_RAW, CORE_?_INTR_ENA, CORE_?_INTR_CLR, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PC, CORE_?_AREA_SP, CORE_?_SP_MIN, CORE_?_SP_MAX, CORE_?_SP_PC, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_DEBUG_MODE, CORE_?_TRACE_LOCKUP_CAUSE_0, CORE_?_TRACE_LOCKUP_TVAL_0, CORE_?_TRACE_LOCKUP_IADDR_0, CORE_?_TRACE_LOCKUP_PRIV_0, CORE_?_TRACE_LOCKUP_CAUSE_1, CORE_?_TRACE_LOCKUP_TVAL_1, CORE_?_TRACE_LOCKUP_IADDR_1, CORE_?_TRACE_LOCKUP_PRIV_1"]
    #[inline(always)]
    pub const fn cpu(&self, n: usize) -> &CPU {
        &self.cpu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x110 - Cluster CPU%s, containing CORE_?_MONTR_ENA, CORE_?_INTR_RAW, CORE_?_INTR_ENA, CORE_?_INTR_CLR, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PC, CORE_?_AREA_SP, CORE_?_SP_MIN, CORE_?_SP_MAX, CORE_?_SP_PC, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_DEBUG_MODE, CORE_?_TRACE_LOCKUP_CAUSE_0, CORE_?_TRACE_LOCKUP_TVAL_0, CORE_?_TRACE_LOCKUP_IADDR_0, CORE_?_TRACE_LOCKUP_PRIV_0, CORE_?_TRACE_LOCKUP_CAUSE_1, CORE_?_TRACE_LOCKUP_TVAL_1, CORE_?_TRACE_LOCKUP_IADDR_1, CORE_?_TRACE_LOCKUP_PRIV_1"]
    #[inline(always)]
    pub fn cpu_iter(&self) -> impl Iterator<Item = &CPU> {
        self.cpu.iter()
    }
    #[doc = "0x110 - exception monitor status register"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor_0(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 {
        &self.core_x_iram0_dram0_exception_monitor_0
    }
    #[doc = "0x114 - Register clock control"]
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
#[doc = "Cluster CPU%s, containing CORE_?_MONTR_ENA, CORE_?_INTR_RAW, CORE_?_INTR_ENA, CORE_?_INTR_CLR, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PC, CORE_?_AREA_SP, CORE_?_SP_MIN, CORE_?_SP_MAX, CORE_?_SP_PC, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_DEBUG_MODE, CORE_?_TRACE_LOCKUP_CAUSE_0, CORE_?_TRACE_LOCKUP_TVAL_0, CORE_?_TRACE_LOCKUP_IADDR_0, CORE_?_TRACE_LOCKUP_PRIV_0, CORE_?_TRACE_LOCKUP_CAUSE_1, CORE_?_TRACE_LOCKUP_TVAL_1, CORE_?_TRACE_LOCKUP_IADDR_1, CORE_?_TRACE_LOCKUP_PRIV_1"]
pub use self::cpu::CPU;
#[doc = r"Cluster"]
#[doc = "Cluster CPU%s, containing CORE_?_MONTR_ENA, CORE_?_INTR_RAW, CORE_?_INTR_ENA, CORE_?_INTR_CLR, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PC, CORE_?_AREA_SP, CORE_?_SP_MIN, CORE_?_SP_MAX, CORE_?_SP_PC, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_DEBUG_MODE, CORE_?_TRACE_LOCKUP_CAUSE_0, CORE_?_TRACE_LOCKUP_TVAL_0, CORE_?_TRACE_LOCKUP_IADDR_0, CORE_?_TRACE_LOCKUP_PRIV_0, CORE_?_TRACE_LOCKUP_CAUSE_1, CORE_?_TRACE_LOCKUP_TVAL_1, CORE_?_TRACE_LOCKUP_IADDR_1, CORE_?_TRACE_LOCKUP_PRIV_1"]
pub mod cpu;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 (rw) register accessor: exception monitor status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor_0`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "exception monitor status register"]
pub mod core_x_iram0_dram0_exception_monitor_0;
#[doc = "CLOCK_GATE (rw) register accessor: Register clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Register clock control"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
