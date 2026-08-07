#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    core_0_intr_map: [CORE_0_INTR_MAP; 98],
    core_0_intr_status: [CORE_0_INTR_STATUS; 3],
    core_0_intr_status3: CORE_0_INTR_STATUS3,
    core_0_src_pass_in_sec_status: [CORE_0_SRC_PASS_IN_SEC_STATUS; 3],
    core_0_src_pass_in_sec_status3: CORE_0_SRC_PASS_IN_SEC_STATUS3,
    sig_idx_assert_in_sec: SIG_IDX_ASSERT_IN_SEC,
    secure_status: SECURE_STATUS,
    clock_gate: CLOCK_GATE,
    _reserved8: [u8; 0x0648],
    interrupt_date: INTERRUPT_DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x188 - "]
    #[inline(always)]
    pub const fn core_0_intr_map(&self, n: usize) -> &CORE_0_INTR_MAP {
        &self.core_0_intr_map[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x188 - "]
    #[inline(always)]
    pub fn core_0_intr_map_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_MAP> {
        self.core_0_intr_map.iter()
    }
    #[doc = "0x188..0x194 - Status of interrupt sources within a 32-bit window"]
    #[inline(always)]
    pub const fn core_0_intr_status(&self, n: usize) -> &CORE_0_INTR_STATUS {
        &self.core_0_intr_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x188..0x194 - Status of interrupt sources within a 32-bit window"]
    #[inline(always)]
    pub fn core_0_intr_status_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_STATUS> {
        self.core_0_intr_status.iter()
    }
    #[doc = "0x194 - Status register for interrupt sources 96 ~ 97"]
    #[inline(always)]
    pub const fn core_0_intr_status3(&self) -> &CORE_0_INTR_STATUS3 {
        &self.core_0_intr_status3
    }
    #[doc = "0x198..0x1a4 - PASS_IN_SEC status for interrupt sources"]
    #[inline(always)]
    pub const fn core_0_src_pass_in_sec_status(&self, n: usize) -> &CORE_0_SRC_PASS_IN_SEC_STATUS {
        &self.core_0_src_pass_in_sec_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x198..0x1a4 - PASS_IN_SEC status for interrupt sources"]
    #[inline(always)]
    pub fn core_0_src_pass_in_sec_status_iter(
        &self,
    ) -> impl Iterator<Item = &CORE_0_SRC_PASS_IN_SEC_STATUS> {
        self.core_0_src_pass_in_sec_status.iter()
    }
    #[doc = "0x1a4 - PASS_IN_SEC status register for interrupt sources 96 ~ 97"]
    #[inline(always)]
    pub const fn core_0_src_pass_in_sec_status3(&self) -> &CORE_0_SRC_PASS_IN_SEC_STATUS3 {
        &self.core_0_src_pass_in_sec_status3
    }
    #[doc = "0x1a8 - reserved"]
    #[inline(always)]
    pub const fn sig_idx_assert_in_sec(&self) -> &SIG_IDX_ASSERT_IN_SEC {
        &self.sig_idx_assert_in_sec
    }
    #[doc = "0x1ac - reserved"]
    #[inline(always)]
    pub const fn secure_status(&self) -> &SECURE_STATUS {
        &self.secure_status
    }
    #[doc = "0x1b0 - Interrupt clock gating configure register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
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
#[doc = "CORE_0_INTR_STATUS (r) register accessor: Status of interrupt sources within a 32-bit window\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_status`] module"]
pub type CORE_0_INTR_STATUS = crate::Reg<core_0_intr_status::CORE_0_INTR_STATUS_SPEC>;
#[doc = "Status of interrupt sources within a 32-bit window"]
pub mod core_0_intr_status;
#[doc = "CORE_0_INTR_STATUS3 (r) register accessor: Status register for interrupt sources 96 ~ 97\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_status3`] module"]
pub type CORE_0_INTR_STATUS3 = crate::Reg<core_0_intr_status3::CORE_0_INTR_STATUS3_SPEC>;
#[doc = "Status register for interrupt sources 96 ~ 97"]
pub mod core_0_intr_status3;
#[doc = "CORE_0_SRC_PASS_IN_SEC_STATUS (r) register accessor: PASS_IN_SEC status for interrupt sources\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_src_pass_in_sec_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_src_pass_in_sec_status`] module"]
pub type CORE_0_SRC_PASS_IN_SEC_STATUS =
    crate::Reg<core_0_src_pass_in_sec_status::CORE_0_SRC_PASS_IN_SEC_STATUS_SPEC>;
#[doc = "PASS_IN_SEC status for interrupt sources"]
pub mod core_0_src_pass_in_sec_status;
#[doc = "CORE_0_SRC_PASS_IN_SEC_STATUS3 (r) register accessor: PASS_IN_SEC status register for interrupt sources 96 ~ 97\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_src_pass_in_sec_status3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_src_pass_in_sec_status3`] module"]
pub type CORE_0_SRC_PASS_IN_SEC_STATUS3 =
    crate::Reg<core_0_src_pass_in_sec_status3::CORE_0_SRC_PASS_IN_SEC_STATUS3_SPEC>;
#[doc = "PASS_IN_SEC status register for interrupt sources 96 ~ 97"]
pub mod core_0_src_pass_in_sec_status3;
#[doc = "SIG_IDX_ASSERT_IN_SEC (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sig_idx_assert_in_sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sig_idx_assert_in_sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sig_idx_assert_in_sec`] module"]
pub type SIG_IDX_ASSERT_IN_SEC = crate::Reg<sig_idx_assert_in_sec::SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = "reserved"]
pub mod sig_idx_assert_in_sec;
#[doc = "SECURE_STATUS (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure_status`] module"]
pub type SECURE_STATUS = crate::Reg<secure_status::SECURE_STATUS_SPEC>;
#[doc = "reserved"]
pub mod secure_status;
#[doc = "CLOCK_GATE (rw) register accessor: Interrupt clock gating configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Interrupt clock gating configure register"]
pub mod clock_gate;
pub use crate::dma::{date as interrupt_date, DATE as INTERRUPT_DATE};
