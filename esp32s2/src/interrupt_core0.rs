#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    core_0_intr_map: [CORE_0_INTR_MAP; 95],
    core_0_intr_status: [CORE_0_INTR_STATUS; 3],
    clock_gate: CLOCK_GATE,
    _reserved3: [u8; 0x0e70],
    reg_date: REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x17c - "]
    #[inline(always)]
    pub const fn core_0_intr_map(&self, n: usize) -> &CORE_0_INTR_MAP {
        &self.core_0_intr_map[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x17c - "]
    #[inline(always)]
    pub fn core_0_intr_map_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_MAP> {
        self.core_0_intr_map.iter()
    }
    #[doc = "0x17c..0x188 - Interrupt status register %s"]
    #[inline(always)]
    pub const fn core_0_intr_status(&self, n: usize) -> &CORE_0_INTR_STATUS {
        &self.core_0_intr_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x17c..0x188 - Interrupt status register %s"]
    #[inline(always)]
    pub fn core_0_intr_status_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_STATUS> {
        self.core_0_intr_status.iter()
    }
    #[doc = "0x188 - NMI interrupt signals mask register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xffc - Version control register"]
    #[inline(always)]
    pub const fn reg_date(&self) -> &REG_DATE {
        &self.reg_date
    }
}
#[doc = "CORE_0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_map`] module"]
pub type CORE_0_INTR_MAP = crate::Reg<core_0_intr_map::CORE_0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod core_0_intr_map;
#[doc = "CORE_0_INTR_STATUS (r) register accessor: Interrupt status register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_status`] module"]
pub type CORE_0_INTR_STATUS = crate::Reg<core_0_intr_status::CORE_0_INTR_STATUS_SPEC>;
#[doc = "Interrupt status register %s"]
pub mod core_0_intr_status;
#[doc = "CLOCK_GATE (rw) register accessor: NMI interrupt signals mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "NMI interrupt signals mask register"]
pub mod clock_gate;
pub use crate::aes::date as reg_date;
pub use crate::aes::DATE as REG_DATE;
