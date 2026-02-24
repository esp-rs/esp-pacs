#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_cpu: [u8; 0x9c],
    _reserved1: [u8; 0x64],
    core_x_iram0_dram0_exception_monitor: [CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR; 2],
    clock_gate: CLOCK_GATE,
    _reserved3: [u8; 0x02f0],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x9c - Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_MONTR_ENA, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
    #[inline(always)]
    pub const fn cpu(&self, n: usize) -> &CPU {
        #[allow(clippy::no_effect)]
        [(); 1][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x9c - Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_MONTR_ENA, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
    #[inline(always)]
    pub fn cpu_iter(&self) -> impl Iterator<Item = &CPU> {
        (0..1).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156 * n).cast() })
    }
    #[doc = "0x68 - exception monitor status register6"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_4(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(104).cast() }
    }
    #[doc = "0x6c - exception monitor status register7"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_5(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x100..0x108 - `¯\\_(ツ)_/¯`"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor(
        &self,
        n: usize,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR {
        &self.core_x_iram0_dram0_exception_monitor[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x108 - `¯\\_(ツ)_/¯`"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_exception_monitor_iter(
        &self,
    ) -> impl Iterator<Item = &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR> {
        self.core_x_iram0_dram0_exception_monitor.iter()
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
#[doc = "Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_MONTR_ENA, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
pub use self::cpu::CPU;
#[doc = r"Cluster"]
#[doc = "Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_MONTR_ENA, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
pub mod cpu;
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
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR (rw) register accessor: `¯\\_(ツ)_/¯`\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR =
    crate::Reg<core_x_iram0_dram0_exception_monitor::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_SPEC>;
#[doc = "`¯\\_(ツ)_/¯`"]
pub mod core_x_iram0_dram0_exception_monitor;
#[doc = "CLOCK_GATE (rw) register accessor: Register clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Register clock control"]
pub mod clock_gate;
pub use crate::aes::date;
pub use crate::aes::DATE;
