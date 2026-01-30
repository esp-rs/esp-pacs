#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_clock_gate: [u8; 0x0150],
    core_0_intr_status: [CORE_0_INTR_STATUS; 3],
    _reserved2: [u8; 0x06a0],
    interrupt_date: INTERRUPT_DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x150 - "]
    #[inline(always)]
    pub const fn core_0_intr_map(&self, n: usize) -> &CORE_0_INTR_MAP {
        #[allow(clippy::no_effect)]
        [(); 84][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x150 - "]
    #[inline(always)]
    pub fn core_0_intr_map_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_MAP> {
        (0..84).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() })
    }
    #[doc = "0x14c - Interrupt clock gating configure register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x150..0x15c - Represents the status of the interrupt sources. Each bit corresponds to one interrupt source"]
    #[inline(always)]
    pub const fn core_0_intr_status(&self, n: usize) -> &CORE_0_INTR_STATUS {
        &self.core_0_intr_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x15c - Represents the status of the interrupt sources. Each bit corresponds to one interrupt source"]
    #[inline(always)]
    pub fn core_0_intr_status_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_STATUS> {
        self.core_0_intr_status.iter()
    }
    #[doc = "0x7fc - Version control register"]
    #[inline(always)]
    pub const fn interrupt_date(&self) -> &INTERRUPT_DATE {
        &self.interrupt_date
    }
}
#[doc = "CORE_0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_map`] module"]
pub type CORE_0_INTR_MAP = crate::Reg<core_0_intr_map::CORE_0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod core_0_intr_map;
#[doc = "CORE_0_INTR_STATUS (r) register accessor: Represents the status of the interrupt sources. Each bit corresponds to one interrupt source\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_status`] module"]
pub type CORE_0_INTR_STATUS = crate::Reg<core_0_intr_status::CORE_0_INTR_STATUS_SPEC>;
#[doc = "Represents the status of the interrupt sources. Each bit corresponds to one interrupt source"]
pub mod core_0_intr_status;
#[doc = "CLOCK_GATE (rw) register accessor: Interrupt clock gating configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Interrupt clock gating configure register"]
pub mod clock_gate;
pub use crate::aes::date as interrupt_date;
pub use crate::aes::DATE as INTERRUPT_DATE;
