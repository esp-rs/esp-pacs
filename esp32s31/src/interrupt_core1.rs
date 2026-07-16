#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    core_1_intr_map: [CORE_1_INTR_MAP; 169],
    sig_idx_assert_in_sec: SIG_IDX_ASSERT_IN_SEC,
    core_1_intr_status: [CORE_1_INTR_STATUS; 5],
    core_1_intr_status5: CORE_1_INTR_STATUS5,
    src_pass_in_s_status: [SRC_PASS_IN_S_STATUS; 5],
    src_pass_in_s_status_5: SRC_PASS_IN_S_STATUS_5,
    src_pass_in_m_status: [SRC_PASS_IN_M_STATUS; 5],
    src_pass_in_m_status_5: SRC_PASS_IN_M_STATUS_5,
    config_state: CONFIG_STATE,
    s_status: S_STATUS,
    m_status: M_STATUS,
    clock_gate: CLOCK_GATE,
    _reserved12: [u8; 0x04fc],
    interrupt_date: INTERRUPT_DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x2a4 - Interrupt-matrix destination and privilege routing for source %s"]
    #[inline(always)]
    pub const fn core_1_intr_map(&self, n: usize) -> &CORE_1_INTR_MAP {
        &self.core_1_intr_map[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x2a4 - Interrupt-matrix destination and privilege routing for source %s"]
    #[inline(always)]
    pub fn core_1_intr_map_iter(&self) -> impl Iterator<Item = &CORE_1_INTR_MAP> {
        self.core_1_intr_map.iter()
    }
    #[doc = "0x2a4 - "]
    #[inline(always)]
    pub const fn sig_idx_assert_in_sec(&self) -> &SIG_IDX_ASSERT_IN_SEC {
        &self.sig_idx_assert_in_sec
    }
    #[doc = "0x2a8..0x2bc - "]
    #[inline(always)]
    pub const fn core_1_intr_status(&self, n: usize) -> &CORE_1_INTR_STATUS {
        &self.core_1_intr_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2a8..0x2bc - "]
    #[inline(always)]
    pub fn core_1_intr_status_iter(&self) -> impl Iterator<Item = &CORE_1_INTR_STATUS> {
        self.core_1_intr_status.iter()
    }
    #[doc = "0x2bc - "]
    #[inline(always)]
    pub const fn core_1_intr_status5(&self) -> &CORE_1_INTR_STATUS5 {
        &self.core_1_intr_status5
    }
    #[doc = "0x2c0..0x2d4 - "]
    #[inline(always)]
    pub const fn src_pass_in_s_status(&self, n: usize) -> &SRC_PASS_IN_S_STATUS {
        &self.src_pass_in_s_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c0..0x2d4 - "]
    #[inline(always)]
    pub fn src_pass_in_s_status_iter(&self) -> impl Iterator<Item = &SRC_PASS_IN_S_STATUS> {
        self.src_pass_in_s_status.iter()
    }
    #[doc = "0x2d4 - "]
    #[inline(always)]
    pub const fn src_pass_in_s_status_5(&self) -> &SRC_PASS_IN_S_STATUS_5 {
        &self.src_pass_in_s_status_5
    }
    #[doc = "0x2d8..0x2ec - "]
    #[inline(always)]
    pub const fn src_pass_in_m_status(&self, n: usize) -> &SRC_PASS_IN_M_STATUS {
        &self.src_pass_in_m_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2d8..0x2ec - "]
    #[inline(always)]
    pub fn src_pass_in_m_status_iter(&self) -> impl Iterator<Item = &SRC_PASS_IN_M_STATUS> {
        self.src_pass_in_m_status.iter()
    }
    #[doc = "0x2ec - "]
    #[inline(always)]
    pub const fn src_pass_in_m_status_5(&self) -> &SRC_PASS_IN_M_STATUS_5 {
        &self.src_pass_in_m_status_5
    }
    #[doc = "0x2f0 - "]
    #[inline(always)]
    pub const fn config_state(&self) -> &CONFIG_STATE {
        &self.config_state
    }
    #[doc = "0x2f4 - "]
    #[inline(always)]
    pub const fn s_status(&self) -> &S_STATUS {
        &self.s_status
    }
    #[doc = "0x2f8 - "]
    #[inline(always)]
    pub const fn m_status(&self) -> &M_STATUS {
        &self.m_status
    }
    #[doc = "0x2fc - "]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x7fc - "]
    #[inline(always)]
    pub const fn interrupt_date(&self) -> &INTERRUPT_DATE {
        &self.interrupt_date
    }
}
#[doc = "CORE_1_INTR_MAP (rw) register accessor: Interrupt-matrix destination and privilege routing for source %s\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_map`] module"]
pub type CORE_1_INTR_MAP = crate::Reg<core_1_intr_map::CORE_1_INTR_MAP_SPEC>;
#[doc = "Interrupt-matrix destination and privilege routing for source %s"]
pub mod core_1_intr_map;
#[doc = "SIG_IDX_ASSERT_IN_SEC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sig_idx_assert_in_sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sig_idx_assert_in_sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sig_idx_assert_in_sec`] module"]
pub type SIG_IDX_ASSERT_IN_SEC = crate::Reg<sig_idx_assert_in_sec::SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = ""]
pub mod sig_idx_assert_in_sec;
#[doc = "CORE_1_INTR_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_status`] module"]
pub type CORE_1_INTR_STATUS = crate::Reg<core_1_intr_status::CORE_1_INTR_STATUS_SPEC>;
#[doc = ""]
pub mod core_1_intr_status;
#[doc = "CORE_1_INTR_STATUS5 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_status5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_status5`] module"]
pub type CORE_1_INTR_STATUS5 = crate::Reg<core_1_intr_status5::CORE_1_INTR_STATUS5_SPEC>;
#[doc = ""]
pub mod core_1_intr_status5;
#[doc = "SRC_PASS_IN_S_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_s_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_s_status`] module"]
pub type SRC_PASS_IN_S_STATUS = crate::Reg<src_pass_in_s_status::SRC_PASS_IN_S_STATUS_SPEC>;
#[doc = ""]
pub mod src_pass_in_s_status;
#[doc = "SRC_PASS_IN_S_STATUS_5 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_s_status_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_s_status_5`] module"]
pub type SRC_PASS_IN_S_STATUS_5 = crate::Reg<src_pass_in_s_status_5::SRC_PASS_IN_S_STATUS_5_SPEC>;
#[doc = ""]
pub mod src_pass_in_s_status_5;
#[doc = "SRC_PASS_IN_M_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_m_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_m_status`] module"]
pub type SRC_PASS_IN_M_STATUS = crate::Reg<src_pass_in_m_status::SRC_PASS_IN_M_STATUS_SPEC>;
#[doc = ""]
pub mod src_pass_in_m_status;
#[doc = "SRC_PASS_IN_M_STATUS_5 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_m_status_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_m_status_5`] module"]
pub type SRC_PASS_IN_M_STATUS_5 = crate::Reg<src_pass_in_m_status_5::SRC_PASS_IN_M_STATUS_5_SPEC>;
#[doc = ""]
pub mod src_pass_in_m_status_5;
#[doc = "CONFIG_STATE (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`config_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_state`] module"]
pub type CONFIG_STATE = crate::Reg<config_state::CONFIG_STATE_SPEC>;
#[doc = ""]
pub mod config_state;
#[doc = "S_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`s_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_status`] module"]
pub type S_STATUS = crate::Reg<s_status::S_STATUS_SPEC>;
#[doc = ""]
pub mod s_status;
#[doc = "M_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`m_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_status`] module"]
pub type M_STATUS = crate::Reg<m_status::M_STATUS_SPEC>;
#[doc = ""]
pub mod m_status;
#[doc = "CLOCK_GATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = ""]
pub mod clock_gate;
#[doc = "INTERRUPT_DATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_date`] module"]
pub type INTERRUPT_DATE = crate::Reg<interrupt_date::INTERRUPT_DATE_SPEC>;
#[doc = ""]
pub mod interrupt_date;
