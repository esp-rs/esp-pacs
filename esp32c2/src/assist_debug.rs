#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu: [CPU; 1],
    clock_gate: CLOCK_GATE,
    _reserved2: [u8; 0x01c8],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_MONTR_ENA, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
    #[inline(always)]
    pub const fn cpu(&self, n: usize) -> &CPU {
        &self.cpu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_MONTR_ENA, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
    #[inline(always)]
    pub fn cpu_iter(&self) -> impl Iterator<Item = &CPU> {
        self.cpu.iter()
    }
    #[doc = "0x30 - clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x1fc - version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_MONTR_ENA, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
pub use self::cpu::CPU;
#[doc = r"Cluster"]
#[doc = "Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_MONTR_ENA, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
pub mod cpu;
#[doc = "CLOCK_GATE (rw) register accessor: clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gate register"]
pub mod clock_gate;
pub use crate::apb_ctrl::date;
pub use crate::apb_ctrl::DATE;
