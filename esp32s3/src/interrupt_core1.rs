#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    core_1_intr_map: [CORE_1_INTR_MAP; 99],
    core_1_intr_status: [CORE_1_INTR_STATUS; 4],
    clock_gate: CLOCK_GATE,
    _reserved3: [u8; 0x065c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x800..0x98c - "]
    #[inline(always)]
    pub const fn core_1_intr_map(&self, n: usize) -> &CORE_1_INTR_MAP {
        &self.core_1_intr_map[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x98c - "]
    #[inline(always)]
    pub fn core_1_intr_map_iter(&self) -> impl Iterator<Item = &CORE_1_INTR_MAP> {
        self.core_1_intr_map.iter()
    }
    #[doc = "0x98c..0x99c - interrupt status register"]
    #[inline(always)]
    pub const fn core_1_intr_status(&self, n: usize) -> &CORE_1_INTR_STATUS {
        &self.core_1_intr_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98c..0x99c - interrupt status register"]
    #[inline(always)]
    pub fn core_1_intr_status_iter(&self) -> impl Iterator<Item = &CORE_1_INTR_STATUS> {
        self.core_1_intr_status.iter()
    }
    #[doc = "0x99c - clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xffc - version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CORE_1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_map`] module"]
pub type CORE_1_INTR_MAP = crate::Reg<core_1_intr_map::CORE_1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod core_1_intr_map;
#[doc = "CORE_1_INTR_STATUS (r) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_status`] module"]
pub type CORE_1_INTR_STATUS = crate::Reg<core_1_intr_status::CORE_1_INTR_STATUS_SPEC>;
#[doc = "interrupt status register"]
pub mod core_1_intr_status;
#[doc = "CLOCK_GATE (rw) register accessor: clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
