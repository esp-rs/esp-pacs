#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    m0_mode_ctrl: M0_MODE_CTRL,
    m1_mode_ctrl: M1_MODE_CTRL,
    _reserved2: [u8; 0x04],
    m_mode_ctrl: [M_MODE_CTRL; 13],
    _reserved3: [u8; 0x10],
    m0_mode_ctrl: M0_MODE_CTRL,
    m1_mode_ctrl: M0_MODE_CTRL,
    m2_mode_ctrl: M0_MODE_CTRL,
    m3_mode_ctrl: M0_MODE_CTRL,
    m4_mode_ctrl: M0_MODE_CTRL,
    m5_mode_ctrl: M0_MODE_CTRL,
    m6_mode_ctrl: M0_MODE_CTRL,
    m7_mode_ctrl: M0_MODE_CTRL,
    m8_mode_ctrl: M0_MODE_CTRL,
    m9_mode_ctrl: M0_MODE_CTRL,
    m10_mode_ctrl: M0_MODE_CTRL,
    m11_mode_ctrl: M0_MODE_CTRL,
    m12_mode_ctrl: M0_MODE_CTRL,
    m13_mode_ctrl: M0_MODE_CTRL,
    m14_mode_ctrl: M0_MODE_CTRL,
    m15_mode_ctrl: M0_MODE_CTRL,
    m16_mode_ctrl: M0_MODE_CTRL,
    m17_mode_ctrl: M0_MODE_CTRL,
    m18_mode_ctrl: M0_MODE_CTRL,
    m19_mode_ctrl: M0_MODE_CTRL,
    m20_mode_ctrl: M0_MODE_CTRL,
    m21_mode_ctrl: M0_MODE_CTRL,
    m22_mode_ctrl: M0_MODE_CTRL,
    m23_mode_ctrl: M0_MODE_CTRL,
    m24_mode_ctrl: M0_MODE_CTRL,
    m25_mode_ctrl: M0_MODE_CTRL,
    m26_mode_ctrl: M0_MODE_CTRL,
    m27_mode_ctrl: M0_MODE_CTRL,
    m28_mode_ctrl: M0_MODE_CTRL,
    m29_mode_ctrl: M0_MODE_CTRL,
    m30_mode_ctrl: M0_MODE_CTRL,
    m31_mode_ctrl: M0_MODE_CTRL,
    m32_mode_ctrl: M0_MODE_CTRL,
    m33_mode_ctrl: M0_MODE_CTRL,
    m34_mode_ctrl: M0_MODE_CTRL,
    m35_mode_ctrl: M0_MODE_CTRL,
    m36_mode_ctrl: M0_MODE_CTRL,
    m37_mode_ctrl: M0_MODE_CTRL,
    m38_mode_ctrl: M0_MODE_CTRL,
    m39_mode_ctrl: M0_MODE_CTRL,
    m40_mode_ctrl: M0_MODE_CTRL,
    m41_mode_ctrl: M0_MODE_CTRL,
    m42_mode_ctrl: M0_MODE_CTRL,
    m43_mode_ctrl: M0_MODE_CTRL,
    m44_mode_ctrl: M0_MODE_CTRL,
    m45_mode_ctrl: M0_MODE_CTRL,
    m46_mode_ctrl: M0_MODE_CTRL,
    m47_mode_ctrl: M0_MODE_CTRL,
    m48_mode_ctrl: M0_MODE_CTRL,
    m49_mode_ctrl: M0_MODE_CTRL,
    m50_mode_ctrl: M0_MODE_CTRL,
    m51_mode_ctrl: M0_MODE_CTRL,
    m52_mode_ctrl: M0_MODE_CTRL,
    m53_mode_ctrl: M0_MODE_CTRL,
    m54_mode_ctrl: M0_MODE_CTRL,
    m55_mode_ctrl: M0_MODE_CTRL,
    m56_mode_ctrl: M0_MODE_CTRL,
    m57_mode_ctrl: M0_MODE_CTRL,
    m58_mode_ctrl: M0_MODE_CTRL,
    m59_mode_ctrl: M0_MODE_CTRL,
    m60_mode_ctrl: M0_MODE_CTRL,
    m61_mode_ctrl: M0_MODE_CTRL,
    m62_mode_ctrl: M0_MODE_CTRL,
    m63_mode_ctrl: M0_MODE_CTRL,
    m64_mode_ctrl: M0_MODE_CTRL,
    m65_mode_ctrl: M0_MODE_CTRL,
    m66_mode_ctrl: M0_MODE_CTRL,
    m67_mode_ctrl: M0_MODE_CTRL,
    _reserved71: [u8; 0xa0],
    supervisor_priv_sel: SUPERVISOR_PRIV_SEL,
    _reserved72: [u8; 0x01f4],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - TEE mode control register"]
    #[inline(always)]
    pub const fn m0_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m0_mode_ctrl
    }
    #[doc = "0x04 - TEE mode control register"]
    #[inline(always)]
    pub const fn m1_mode_ctrl(&self) -> &M1_MODE_CTRL {
        &self.m1_mode_ctrl
    }
    #[doc = "0x0c..0x40 - TEE mode control register"]
    #[inline(always)]
    pub const fn m_mode_ctrl(&self, n: usize) -> &M_MODE_CTRL {
        &self.m_mode_ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x40 - TEE mode control register"]
    #[inline(always)]
    pub fn m_mode_ctrl_iter(&self) -> impl Iterator<Item = &M_MODE_CTRL> {
        self.m_mode_ctrl.iter()
    }
    #[doc = "0x0c - TEE mode control register"]
    #[inline(always)]
    pub const fn m0_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(0)
    }
    #[doc = "0x10 - TEE mode control register"]
    #[inline(always)]
    pub const fn m1_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(1)
    }
    #[doc = "0x14 - TEE mode control register"]
    #[inline(always)]
    pub const fn m2_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(2)
    }
    #[doc = "0x18 - TEE mode control register"]
    #[inline(always)]
    pub const fn m3_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(3)
    }
    #[doc = "0x1c - TEE mode control register"]
    #[inline(always)]
    pub const fn m4_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(4)
    }
    #[doc = "0x20 - TEE mode control register"]
    #[inline(always)]
    pub const fn m5_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(5)
    }
    #[doc = "0x24 - TEE mode control register"]
    #[inline(always)]
    pub const fn m6_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(6)
    }
    #[doc = "0x28 - TEE mode control register"]
    #[inline(always)]
    pub const fn m7_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(7)
    }
    #[doc = "0x2c - TEE mode control register"]
    #[inline(always)]
    pub const fn m8_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(8)
    }
    #[doc = "0x30 - TEE mode control register"]
    #[inline(always)]
    pub const fn m9_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(9)
    }
    #[doc = "0x34 - TEE mode control register"]
    #[inline(always)]
    pub const fn m10_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(10)
    }
    #[doc = "0x38 - TEE mode control register"]
    #[inline(always)]
    pub const fn m11_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(11)
    }
    #[doc = "0x3c - TEE mode control register"]
    #[inline(always)]
    pub const fn m12_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(12)
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m0_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m0_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m1_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m1_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m2_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m2_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m3_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m3_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m4_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m4_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m5_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m5_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m6_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m6_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m7_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m7_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m8_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m8_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m9_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m9_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m10_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m10_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m11_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m11_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m12_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m12_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m13_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m13_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m14_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m14_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m15_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m15_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m16_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m16_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m17_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m17_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m18_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m18_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m19_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m19_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m20_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m20_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m21_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m21_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m22_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m22_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m23_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m23_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m24_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m24_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m25_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m25_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m26_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m26_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m27_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m27_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m28_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m28_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m29_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m29_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m30_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m30_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m31_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m31_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m32_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m32_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m33_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m33_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m34_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m34_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m35_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m35_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m36_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m36_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m37_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m37_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m38_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m38_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m39_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m39_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m40_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m40_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m41_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m41_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m42_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m42_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m43_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m43_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m44_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m44_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m45_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m45_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m46_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m46_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m47_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m47_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m48_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m48_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m49_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m49_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m50_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m50_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m51_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m51_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m52_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m52_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m53_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m53_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m54_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m54_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m55_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m55_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m56_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m56_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m57_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m57_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m58_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m58_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m59_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m59_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m60_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m60_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m61_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m61_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m62_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m62_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m63_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m63_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m64_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m64_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m65_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m65_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m66_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m66_mode_ctrl
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m67_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m67_mode_ctrl
    }
    #[doc = "0x200 - TEE mode control register"]
    #[inline(always)]
    pub const fn supervisor_priv_sel(&self) -> &SUPERVISOR_PRIV_SEL {
        &self.supervisor_priv_sel
    }
    #[doc = "0x3f8 - Clock gating register"]
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
#[doc = "M0_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_mode_ctrl`] module"]
pub type M0_MODE_CTRL = crate::Reg<m0_mode_ctrl::M0_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m0_mode_ctrl;
#[doc = "M1_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_mode_ctrl`] module"]
pub type M1_MODE_CTRL = crate::Reg<m1_mode_ctrl::M1_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m1_mode_ctrl;
#[doc = "M_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_mode_ctrl`] module"]
pub type M_MODE_CTRL = crate::Reg<m_mode_ctrl::M_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m_mode_ctrl;
pub use m_mode_ctrl;
pub use M_MODE_CTRL;
#[doc = "SUPERVISOR_PRIV_SEL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`supervisor_priv_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`supervisor_priv_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@supervisor_priv_sel`] module"]
pub type SUPERVISOR_PRIV_SEL = crate::Reg<supervisor_priv_sel::SUPERVISOR_PRIV_SEL_SPEC>;
#[doc = "TEE mode control register"]
pub mod supervisor_priv_sel;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
