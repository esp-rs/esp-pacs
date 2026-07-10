#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    core_0_intr_map: [CORE_0_INTR_MAP; 66],
    core_0_intr_status: [CORE_0_INTR_STATUS; 3],
    src_pass_in_sec_status_0: SRC_PASS_IN_SEC_STATUS_0,
    src_pass_in_sec_status_1: SRC_PASS_IN_SEC_STATUS_1,
    src_pass_in_sec_status_2: SRC_PASS_IN_SEC_STATUS_2,
    sig_idx_assert_in_sec: SIG_IDX_ASSERT_IN_SEC,
    secure_status: SECURE_STATUS,
    clock_gate: CLOCK_GATE,
    _reserved8: [u8; 0x06d0],
    interrupt_date: INTERRUPT_DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x108 - "]
    #[inline(always)]
    pub const fn core_0_intr_map(&self, n: usize) -> &CORE_0_INTR_MAP {
        &self.core_0_intr_map[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x108 - "]
    #[inline(always)]
    pub fn core_0_intr_map_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_MAP> {
        self.core_0_intr_map.iter()
    }
    #[doc = "0x108..0x114 - Represents the status of the interrupt sources. Each bit corresponds to one interrupt source"]
    #[inline(always)]
    pub const fn core_0_intr_status(&self, n: usize) -> &CORE_0_INTR_STATUS {
        &self.core_0_intr_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x114 - Represents the status of the interrupt sources. Each bit corresponds to one interrupt source"]
    #[inline(always)]
    pub fn core_0_intr_status_iter(&self) -> impl Iterator<Item = &CORE_0_INTR_STATUS> {
        self.core_0_intr_status.iter()
    }
    #[doc = "0x114 - PASS_IN_SEC status register for interrupt sources 0 ~ 31"]
    #[inline(always)]
    pub const fn src_pass_in_sec_status_0(&self) -> &SRC_PASS_IN_SEC_STATUS_0 {
        &self.src_pass_in_sec_status_0
    }
    #[doc = "0x118 - PASS_IN_SEC status register for interrupt sources 32 ~ 63"]
    #[inline(always)]
    pub const fn src_pass_in_sec_status_1(&self) -> &SRC_PASS_IN_SEC_STATUS_1 {
        &self.src_pass_in_sec_status_1
    }
    #[doc = "0x11c - PASS_IN_SEC status register for interrupt sources 64 ~ 65"]
    #[inline(always)]
    pub const fn src_pass_in_sec_status_2(&self) -> &SRC_PASS_IN_SEC_STATUS_2 {
        &self.src_pass_in_sec_status_2
    }
    #[doc = "0x120 - reserved"]
    #[inline(always)]
    pub const fn sig_idx_assert_in_sec(&self) -> &SIG_IDX_ASSERT_IN_SEC {
        &self.sig_idx_assert_in_sec
    }
    #[doc = "0x124 - reserved"]
    #[inline(always)]
    pub const fn secure_status(&self) -> &SECURE_STATUS {
        &self.secure_status
    }
    #[doc = "0x128 - Interrupt clock gating configure register"]
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
#[doc = "CORE_0_INTR_STATUS (r) register accessor: Represents the status of the interrupt sources. Each bit corresponds to one interrupt source\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_status`] module"]
pub type CORE_0_INTR_STATUS = crate::Reg<core_0_intr_status::CORE_0_INTR_STATUS_SPEC>;
#[doc = "Represents the status of the interrupt sources. Each bit corresponds to one interrupt source"]
pub mod core_0_intr_status;
#[doc = "SRC_PASS_IN_SEC_STATUS_0 (r) register accessor: PASS_IN_SEC status register for interrupt sources 0 ~ 31\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_sec_status_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_sec_status_0`] module"]
pub type SRC_PASS_IN_SEC_STATUS_0 =
    crate::Reg<src_pass_in_sec_status_0::SRC_PASS_IN_SEC_STATUS_0_SPEC>;
#[doc = "PASS_IN_SEC status register for interrupt sources 0 ~ 31"]
pub mod src_pass_in_sec_status_0;
#[doc = "SRC_PASS_IN_SEC_STATUS_1 (r) register accessor: PASS_IN_SEC status register for interrupt sources 32 ~ 63\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_sec_status_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_sec_status_1`] module"]
pub type SRC_PASS_IN_SEC_STATUS_1 =
    crate::Reg<src_pass_in_sec_status_1::SRC_PASS_IN_SEC_STATUS_1_SPEC>;
#[doc = "PASS_IN_SEC status register for interrupt sources 32 ~ 63"]
pub mod src_pass_in_sec_status_1;
#[doc = "SRC_PASS_IN_SEC_STATUS_2 (r) register accessor: PASS_IN_SEC status register for interrupt sources 64 ~ 65\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_sec_status_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_sec_status_2`] module"]
pub type SRC_PASS_IN_SEC_STATUS_2 =
    crate::Reg<src_pass_in_sec_status_2::SRC_PASS_IN_SEC_STATUS_2_SPEC>;
#[doc = "PASS_IN_SEC status register for interrupt sources 64 ~ 65"]
pub mod src_pass_in_sec_status_2;
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
