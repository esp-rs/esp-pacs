#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_core_1_intr: [u8; 0x0210],
    clock_gate: CLOCK_GATE,
    _reserved2: [u8; 0x0c],
    core_1_intr_status4: CORE_1_INTR_STATUS4,
    _reserved3: [u8; 0x04],
    intr_sig_idx_assert_in_sec: INTR_SIG_IDX_ASSERT_IN_SEC,
    intr_sec_status: INTR_SEC_STATUS,
    intr_src_pass_in_sec_status_0: INTR_SRC_PASS_IN_SEC_STATUS_0,
    intr_src_pass_in_sec_status_1: INTR_SRC_PASS_IN_SEC_STATUS_1,
    intr_src_pass_in_sec_status_2: INTR_SRC_PASS_IN_SEC_STATUS_2,
    intr_src_pass_in_sec_status_3: INTR_SRC_PASS_IN_SEC_STATUS_3,
    intr_src_pass_in_sec_status_4: INTR_SRC_PASS_IN_SEC_STATUS_4,
    _reserved10: [u8; 0x01b8],
    interrupt_reg_date: INTERRUPT_REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x20c - "]
    #[inline(always)]
    pub const fn core_1_intr_map(&self, n: usize) -> &CORE_1_INTR_MAP {
        #[allow(clippy::no_effect)]
        [(); 131][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20c - "]
    #[inline(always)]
    pub fn core_1_intr_map_iter(&self) -> impl Iterator<Item = &CORE_1_INTR_MAP> {
        (0..131).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() })
    }
    #[doc = "0x200..0x210 - NA"]
    #[inline(always)]
    pub const fn core_1_intr_status(&self, n: usize) -> &CORE_1_INTR_STATUS {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x210 - NA"]
    #[inline(always)]
    pub fn core_1_intr_status_iter(&self) -> impl Iterator<Item = &CORE_1_INTR_STATUS> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x210 - NA"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x220 - NA"]
    #[inline(always)]
    pub const fn core_1_intr_status4(&self) -> &CORE_1_INTR_STATUS4 {
        &self.core_1_intr_status4
    }
    #[doc = "0x228 - NA"]
    #[inline(always)]
    pub const fn intr_sig_idx_assert_in_sec(&self) -> &INTR_SIG_IDX_ASSERT_IN_SEC {
        &self.intr_sig_idx_assert_in_sec
    }
    #[doc = "0x22c - NA"]
    #[inline(always)]
    pub const fn intr_sec_status(&self) -> &INTR_SEC_STATUS {
        &self.intr_sec_status
    }
    #[doc = "0x230 - NA"]
    #[inline(always)]
    pub const fn intr_src_pass_in_sec_status_0(&self) -> &INTR_SRC_PASS_IN_SEC_STATUS_0 {
        &self.intr_src_pass_in_sec_status_0
    }
    #[doc = "0x234 - NA"]
    #[inline(always)]
    pub const fn intr_src_pass_in_sec_status_1(&self) -> &INTR_SRC_PASS_IN_SEC_STATUS_1 {
        &self.intr_src_pass_in_sec_status_1
    }
    #[doc = "0x238 - NA"]
    #[inline(always)]
    pub const fn intr_src_pass_in_sec_status_2(&self) -> &INTR_SRC_PASS_IN_SEC_STATUS_2 {
        &self.intr_src_pass_in_sec_status_2
    }
    #[doc = "0x23c - NA"]
    #[inline(always)]
    pub const fn intr_src_pass_in_sec_status_3(&self) -> &INTR_SRC_PASS_IN_SEC_STATUS_3 {
        &self.intr_src_pass_in_sec_status_3
    }
    #[doc = "0x240 - NA"]
    #[inline(always)]
    pub const fn intr_src_pass_in_sec_status_4(&self) -> &INTR_SRC_PASS_IN_SEC_STATUS_4 {
        &self.intr_src_pass_in_sec_status_4
    }
    #[doc = "0x3fc - NA"]
    #[inline(always)]
    pub const fn interrupt_reg_date(&self) -> &INTERRUPT_REG_DATE {
        &self.interrupt_reg_date
    }
}
#[doc = "CORE_1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_map`] module"]
pub type CORE_1_INTR_MAP = crate::Reg<core_1_intr_map::CORE_1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod core_1_intr_map;
#[doc = "CORE_1_INTR_STATUS (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_status`] module"]
pub type CORE_1_INTR_STATUS = crate::Reg<core_1_intr_status::CORE_1_INTR_STATUS_SPEC>;
#[doc = "NA"]
pub mod core_1_intr_status;
#[doc = "CLOCK_GATE (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "NA"]
pub mod clock_gate;
#[doc = "CORE_1_INTR_STATUS4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_status4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_status4`] module"]
pub type CORE_1_INTR_STATUS4 = crate::Reg<core_1_intr_status4::CORE_1_INTR_STATUS4_SPEC>;
#[doc = "NA"]
pub mod core_1_intr_status4;
#[doc = "INTR_SIG_IDX_ASSERT_IN_SEC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sig_idx_assert_in_sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_sig_idx_assert_in_sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sig_idx_assert_in_sec`] module"]
pub type INTR_SIG_IDX_ASSERT_IN_SEC =
    crate::Reg<intr_sig_idx_assert_in_sec::INTR_SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = "NA"]
pub mod intr_sig_idx_assert_in_sec;
#[doc = "INTR_SEC_STATUS (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sec_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sec_status`] module"]
pub type INTR_SEC_STATUS = crate::Reg<intr_sec_status::INTR_SEC_STATUS_SPEC>;
#[doc = "NA"]
pub mod intr_sec_status;
#[doc = "INTR_SRC_PASS_IN_SEC_STATUS_0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_src_pass_in_sec_status_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_src_pass_in_sec_status_0`] module"]
pub type INTR_SRC_PASS_IN_SEC_STATUS_0 =
    crate::Reg<intr_src_pass_in_sec_status_0::INTR_SRC_PASS_IN_SEC_STATUS_0_SPEC>;
#[doc = "NA"]
pub mod intr_src_pass_in_sec_status_0;
#[doc = "INTR_SRC_PASS_IN_SEC_STATUS_1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_src_pass_in_sec_status_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_src_pass_in_sec_status_1`] module"]
pub type INTR_SRC_PASS_IN_SEC_STATUS_1 =
    crate::Reg<intr_src_pass_in_sec_status_1::INTR_SRC_PASS_IN_SEC_STATUS_1_SPEC>;
#[doc = "NA"]
pub mod intr_src_pass_in_sec_status_1;
#[doc = "INTR_SRC_PASS_IN_SEC_STATUS_2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_src_pass_in_sec_status_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_src_pass_in_sec_status_2`] module"]
pub type INTR_SRC_PASS_IN_SEC_STATUS_2 =
    crate::Reg<intr_src_pass_in_sec_status_2::INTR_SRC_PASS_IN_SEC_STATUS_2_SPEC>;
#[doc = "NA"]
pub mod intr_src_pass_in_sec_status_2;
#[doc = "INTR_SRC_PASS_IN_SEC_STATUS_3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_src_pass_in_sec_status_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_src_pass_in_sec_status_3`] module"]
pub type INTR_SRC_PASS_IN_SEC_STATUS_3 =
    crate::Reg<intr_src_pass_in_sec_status_3::INTR_SRC_PASS_IN_SEC_STATUS_3_SPEC>;
#[doc = "NA"]
pub mod intr_src_pass_in_sec_status_3;
#[doc = "INTR_SRC_PASS_IN_SEC_STATUS_4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_src_pass_in_sec_status_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_src_pass_in_sec_status_4`] module"]
pub type INTR_SRC_PASS_IN_SEC_STATUS_4 =
    crate::Reg<intr_src_pass_in_sec_status_4::INTR_SRC_PASS_IN_SEC_STATUS_4_SPEC>;
#[doc = "NA"]
pub mod intr_src_pass_in_sec_status_4;
#[doc = "INTERRUPT_REG_DATE (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_reg_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_reg_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_reg_date`] module"]
pub type INTERRUPT_REG_DATE = crate::Reg<interrupt_reg_date::INTERRUPT_REG_DATE_SPEC>;
#[doc = "NA"]
pub mod interrupt_reg_date;
