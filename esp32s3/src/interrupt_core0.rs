#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    core_0_intr_map: [CORE_0_INTR_MAP; 99],
    core_0_intr_status: [CORE_0_INTR_STATUS; 4],
    clock_gate: CLOCK_GATE,
    _reserved3: [u8; 0x065c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x18c - "]
    #[inline(always)]
    pub const fn core_0_intr_map(&self, n: usize) -> &CORE_0_INTR_MAP {
        &self.core_0_intr_map[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x18c - "]
    #[inline(always)]
    pub fn core_0_intr_map_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_MAP> {
        self.core_0_intr_map.iter()
    }
    #[doc = "0x18c..0x19c - interrupt status register"]
    #[inline(always)]
    pub const fn core_0_intr_status(&self, n: usize) -> &CORE_0_INTR_STATUS {
        &self.core_0_intr_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18c..0x19c - interrupt status register"]
    #[inline(always)]
    pub fn core_0_intr_status_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_STATUS> {
        self.core_0_intr_status.iter()
    }
    #[doc = "0x19c - clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x7fc - version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CORE_0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_map`] module"]
pub type CORE_0_INTR_MAP = crate::Reg<core_0_intr_map::CORE_0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod core_0_intr_map;
#[doc = "CORE_0_INTR_STATUS (r) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_status`] module"]
pub type CORE_0_INTR_STATUS = crate::Reg<core_0_intr_status::CORE_0_INTR_STATUS_SPEC>;
#[doc = "interrupt status register"]
pub mod core_0_intr_status;
#[doc = "CLOCK_GATE (rw) register accessor: clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
