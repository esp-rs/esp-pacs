#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    m0_mode_ctrl: M0_MODE_CTRL,
    m1_mode_ctrl: M1_MODE_CTRL,
    m2_mode_ctrl: M2_MODE_CTRL,
    m3_mode_ctrl: M3_MODE_CTRL,
    m4_mode_ctrl: M4_MODE_CTRL,
    m5_mode_ctrl: M5_MODE_CTRL,
    m6_mode_ctrl: M6_MODE_CTRL,
    m7_mode_ctrl: M7_MODE_CTRL,
    m8_mode_ctrl: M8_MODE_CTRL,
    m9_mode_ctrl: M9_MODE_CTRL,
    m10_mode_ctrl: M10_MODE_CTRL,
    m11_mode_ctrl: M11_MODE_CTRL,
    m12_mode_ctrl: M12_MODE_CTRL,
    m13_mode_ctrl: M13_MODE_CTRL,
    m14_mode_ctrl: M14_MODE_CTRL,
    m15_mode_ctrl: M15_MODE_CTRL,
    m16_mode_ctrl: M16_MODE_CTRL,
    m17_mode_ctrl: M17_MODE_CTRL,
    m18_mode_ctrl: M18_MODE_CTRL,
    m19_mode_ctrl: M19_MODE_CTRL,
    m20_mode_ctrl: M20_MODE_CTRL,
    m21_mode_ctrl: M21_MODE_CTRL,
    m22_mode_ctrl: M22_MODE_CTRL,
    m23_mode_ctrl: M23_MODE_CTRL,
    m24_mode_ctrl: M24_MODE_CTRL,
    m25_mode_ctrl: M25_MODE_CTRL,
    m26_mode_ctrl: M26_MODE_CTRL,
    m27_mode_ctrl: M27_MODE_CTRL,
    m28_mode_ctrl: M28_MODE_CTRL,
    m29_mode_ctrl: M29_MODE_CTRL,
    m30_mode_ctrl: M30_MODE_CTRL,
    m31_mode_ctrl: M31_MODE_CTRL,
    clock_gate: CLOCK_GATE,
    _reserved33: [u8; 0x0f78],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Tee mode control register"]
    #[inline(always)]
    pub const fn m0_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m0_mode_ctrl
    }
    #[doc = "0x04 - Tee mode control register"]
    #[inline(always)]
    pub const fn m1_mode_ctrl(&self) -> &M1_MODE_CTRL {
        &self.m1_mode_ctrl
    }
    #[doc = "0x08 - Tee mode control register"]
    #[inline(always)]
    pub const fn m2_mode_ctrl(&self) -> &M2_MODE_CTRL {
        &self.m2_mode_ctrl
    }
    #[doc = "0x0c - Tee mode control register"]
    #[inline(always)]
    pub const fn m3_mode_ctrl(&self) -> &M3_MODE_CTRL {
        &self.m3_mode_ctrl
    }
    #[doc = "0x10 - Tee mode control register"]
    #[inline(always)]
    pub const fn m4_mode_ctrl(&self) -> &M4_MODE_CTRL {
        &self.m4_mode_ctrl
    }
    #[doc = "0x14 - Tee mode control register"]
    #[inline(always)]
    pub const fn m5_mode_ctrl(&self) -> &M5_MODE_CTRL {
        &self.m5_mode_ctrl
    }
    #[doc = "0x18 - Tee mode control register"]
    #[inline(always)]
    pub const fn m6_mode_ctrl(&self) -> &M6_MODE_CTRL {
        &self.m6_mode_ctrl
    }
    #[doc = "0x1c - Tee mode control register"]
    #[inline(always)]
    pub const fn m7_mode_ctrl(&self) -> &M7_MODE_CTRL {
        &self.m7_mode_ctrl
    }
    #[doc = "0x20 - Tee mode control register"]
    #[inline(always)]
    pub const fn m8_mode_ctrl(&self) -> &M8_MODE_CTRL {
        &self.m8_mode_ctrl
    }
    #[doc = "0x24 - Tee mode control register"]
    #[inline(always)]
    pub const fn m9_mode_ctrl(&self) -> &M9_MODE_CTRL {
        &self.m9_mode_ctrl
    }
    #[doc = "0x28 - Tee mode control register"]
    #[inline(always)]
    pub const fn m10_mode_ctrl(&self) -> &M10_MODE_CTRL {
        &self.m10_mode_ctrl
    }
    #[doc = "0x2c - Tee mode control register"]
    #[inline(always)]
    pub const fn m11_mode_ctrl(&self) -> &M11_MODE_CTRL {
        &self.m11_mode_ctrl
    }
    #[doc = "0x30 - Tee mode control register"]
    #[inline(always)]
    pub const fn m12_mode_ctrl(&self) -> &M12_MODE_CTRL {
        &self.m12_mode_ctrl
    }
    #[doc = "0x34 - Tee mode control register"]
    #[inline(always)]
    pub const fn m13_mode_ctrl(&self) -> &M13_MODE_CTRL {
        &self.m13_mode_ctrl
    }
    #[doc = "0x38 - Tee mode control register"]
    #[inline(always)]
    pub const fn m14_mode_ctrl(&self) -> &M14_MODE_CTRL {
        &self.m14_mode_ctrl
    }
    #[doc = "0x3c - Tee mode control register"]
    #[inline(always)]
    pub const fn m15_mode_ctrl(&self) -> &M15_MODE_CTRL {
        &self.m15_mode_ctrl
    }
    #[doc = "0x40 - Tee mode control register"]
    #[inline(always)]
    pub const fn m16_mode_ctrl(&self) -> &M16_MODE_CTRL {
        &self.m16_mode_ctrl
    }
    #[doc = "0x44 - Tee mode control register"]
    #[inline(always)]
    pub const fn m17_mode_ctrl(&self) -> &M17_MODE_CTRL {
        &self.m17_mode_ctrl
    }
    #[doc = "0x48 - Tee mode control register"]
    #[inline(always)]
    pub const fn m18_mode_ctrl(&self) -> &M18_MODE_CTRL {
        &self.m18_mode_ctrl
    }
    #[doc = "0x4c - Tee mode control register"]
    #[inline(always)]
    pub const fn m19_mode_ctrl(&self) -> &M19_MODE_CTRL {
        &self.m19_mode_ctrl
    }
    #[doc = "0x50 - Tee mode control register"]
    #[inline(always)]
    pub const fn m20_mode_ctrl(&self) -> &M20_MODE_CTRL {
        &self.m20_mode_ctrl
    }
    #[doc = "0x54 - Tee mode control register"]
    #[inline(always)]
    pub const fn m21_mode_ctrl(&self) -> &M21_MODE_CTRL {
        &self.m21_mode_ctrl
    }
    #[doc = "0x58 - Tee mode control register"]
    #[inline(always)]
    pub const fn m22_mode_ctrl(&self) -> &M22_MODE_CTRL {
        &self.m22_mode_ctrl
    }
    #[doc = "0x5c - Tee mode control register"]
    #[inline(always)]
    pub const fn m23_mode_ctrl(&self) -> &M23_MODE_CTRL {
        &self.m23_mode_ctrl
    }
    #[doc = "0x60 - Tee mode control register"]
    #[inline(always)]
    pub const fn m24_mode_ctrl(&self) -> &M24_MODE_CTRL {
        &self.m24_mode_ctrl
    }
    #[doc = "0x64 - Tee mode control register"]
    #[inline(always)]
    pub const fn m25_mode_ctrl(&self) -> &M25_MODE_CTRL {
        &self.m25_mode_ctrl
    }
    #[doc = "0x68 - Tee mode control register"]
    #[inline(always)]
    pub const fn m26_mode_ctrl(&self) -> &M26_MODE_CTRL {
        &self.m26_mode_ctrl
    }
    #[doc = "0x6c - Tee mode control register"]
    #[inline(always)]
    pub const fn m27_mode_ctrl(&self) -> &M27_MODE_CTRL {
        &self.m27_mode_ctrl
    }
    #[doc = "0x70 - Tee mode control register"]
    #[inline(always)]
    pub const fn m28_mode_ctrl(&self) -> &M28_MODE_CTRL {
        &self.m28_mode_ctrl
    }
    #[doc = "0x74 - Tee mode control register"]
    #[inline(always)]
    pub const fn m29_mode_ctrl(&self) -> &M29_MODE_CTRL {
        &self.m29_mode_ctrl
    }
    #[doc = "0x78 - Tee mode control register"]
    #[inline(always)]
    pub const fn m30_mode_ctrl(&self) -> &M30_MODE_CTRL {
        &self.m30_mode_ctrl
    }
    #[doc = "0x7c - Tee mode control register"]
    #[inline(always)]
    pub const fn m31_mode_ctrl(&self) -> &M31_MODE_CTRL {
        &self.m31_mode_ctrl
    }
    #[doc = "0x80 - Clock gating register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xffc - Version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "M0_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m0_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m0_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_mode_ctrl`] module"]
pub type M0_MODE_CTRL = crate::Reg<m0_mode_ctrl::M0_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m0_mode_ctrl;
#[doc = "M1_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_mode_ctrl`] module"]
pub type M1_MODE_CTRL = crate::Reg<m1_mode_ctrl::M1_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m1_mode_ctrl;
#[doc = "M2_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_mode_ctrl`] module"]
pub type M2_MODE_CTRL = crate::Reg<m2_mode_ctrl::M2_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m2_mode_ctrl;
#[doc = "M3_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3_mode_ctrl`] module"]
pub type M3_MODE_CTRL = crate::Reg<m3_mode_ctrl::M3_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m3_mode_ctrl;
#[doc = "M4_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4_mode_ctrl`] module"]
pub type M4_MODE_CTRL = crate::Reg<m4_mode_ctrl::M4_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m4_mode_ctrl;
#[doc = "M5_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5_mode_ctrl`] module"]
pub type M5_MODE_CTRL = crate::Reg<m5_mode_ctrl::M5_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m5_mode_ctrl;
#[doc = "M6_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m6_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m6_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m6_mode_ctrl`] module"]
pub type M6_MODE_CTRL = crate::Reg<m6_mode_ctrl::M6_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m6_mode_ctrl;
#[doc = "M7_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m7_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m7_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m7_mode_ctrl`] module"]
pub type M7_MODE_CTRL = crate::Reg<m7_mode_ctrl::M7_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m7_mode_ctrl;
#[doc = "M8_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m8_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m8_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m8_mode_ctrl`] module"]
pub type M8_MODE_CTRL = crate::Reg<m8_mode_ctrl::M8_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m8_mode_ctrl;
#[doc = "M9_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m9_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m9_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m9_mode_ctrl`] module"]
pub type M9_MODE_CTRL = crate::Reg<m9_mode_ctrl::M9_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m9_mode_ctrl;
#[doc = "M10_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m10_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m10_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m10_mode_ctrl`] module"]
pub type M10_MODE_CTRL = crate::Reg<m10_mode_ctrl::M10_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m10_mode_ctrl;
#[doc = "M11_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m11_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m11_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m11_mode_ctrl`] module"]
pub type M11_MODE_CTRL = crate::Reg<m11_mode_ctrl::M11_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m11_mode_ctrl;
#[doc = "M12_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m12_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m12_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m12_mode_ctrl`] module"]
pub type M12_MODE_CTRL = crate::Reg<m12_mode_ctrl::M12_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m12_mode_ctrl;
#[doc = "M13_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m13_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m13_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m13_mode_ctrl`] module"]
pub type M13_MODE_CTRL = crate::Reg<m13_mode_ctrl::M13_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m13_mode_ctrl;
#[doc = "M14_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m14_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m14_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m14_mode_ctrl`] module"]
pub type M14_MODE_CTRL = crate::Reg<m14_mode_ctrl::M14_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m14_mode_ctrl;
#[doc = "M15_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m15_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m15_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m15_mode_ctrl`] module"]
pub type M15_MODE_CTRL = crate::Reg<m15_mode_ctrl::M15_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m15_mode_ctrl;
#[doc = "M16_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m16_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m16_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m16_mode_ctrl`] module"]
pub type M16_MODE_CTRL = crate::Reg<m16_mode_ctrl::M16_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m16_mode_ctrl;
#[doc = "M17_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m17_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m17_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m17_mode_ctrl`] module"]
pub type M17_MODE_CTRL = crate::Reg<m17_mode_ctrl::M17_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m17_mode_ctrl;
#[doc = "M18_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m18_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m18_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m18_mode_ctrl`] module"]
pub type M18_MODE_CTRL = crate::Reg<m18_mode_ctrl::M18_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m18_mode_ctrl;
#[doc = "M19_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m19_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m19_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m19_mode_ctrl`] module"]
pub type M19_MODE_CTRL = crate::Reg<m19_mode_ctrl::M19_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m19_mode_ctrl;
#[doc = "M20_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m20_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m20_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m20_mode_ctrl`] module"]
pub type M20_MODE_CTRL = crate::Reg<m20_mode_ctrl::M20_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m20_mode_ctrl;
#[doc = "M21_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m21_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m21_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m21_mode_ctrl`] module"]
pub type M21_MODE_CTRL = crate::Reg<m21_mode_ctrl::M21_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m21_mode_ctrl;
#[doc = "M22_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m22_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m22_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m22_mode_ctrl`] module"]
pub type M22_MODE_CTRL = crate::Reg<m22_mode_ctrl::M22_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m22_mode_ctrl;
#[doc = "M23_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m23_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m23_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m23_mode_ctrl`] module"]
pub type M23_MODE_CTRL = crate::Reg<m23_mode_ctrl::M23_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m23_mode_ctrl;
#[doc = "M24_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m24_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m24_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m24_mode_ctrl`] module"]
pub type M24_MODE_CTRL = crate::Reg<m24_mode_ctrl::M24_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m24_mode_ctrl;
#[doc = "M25_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m25_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m25_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m25_mode_ctrl`] module"]
pub type M25_MODE_CTRL = crate::Reg<m25_mode_ctrl::M25_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m25_mode_ctrl;
#[doc = "M26_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m26_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m26_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m26_mode_ctrl`] module"]
pub type M26_MODE_CTRL = crate::Reg<m26_mode_ctrl::M26_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m26_mode_ctrl;
#[doc = "M27_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m27_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m27_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m27_mode_ctrl`] module"]
pub type M27_MODE_CTRL = crate::Reg<m27_mode_ctrl::M27_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m27_mode_ctrl;
#[doc = "M28_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m28_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m28_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m28_mode_ctrl`] module"]
pub type M28_MODE_CTRL = crate::Reg<m28_mode_ctrl::M28_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m28_mode_ctrl;
#[doc = "M29_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m29_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m29_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m29_mode_ctrl`] module"]
pub type M29_MODE_CTRL = crate::Reg<m29_mode_ctrl::M29_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m29_mode_ctrl;
#[doc = "M30_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m30_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m30_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m30_mode_ctrl`] module"]
pub type M30_MODE_CTRL = crate::Reg<m30_mode_ctrl::M30_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m30_mode_ctrl;
#[doc = "M31_MODE_CTRL (rw) register accessor: Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m31_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m31_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_mode_ctrl`] module"]
pub type M31_MODE_CTRL = crate::Reg<m31_mode_ctrl::M31_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m31_mode_ctrl;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version register"]
pub mod date;
