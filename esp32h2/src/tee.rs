#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    m_mode_ctrl: [M_MODE_CTRL; 32],
    clock_gate: CLOCK_GATE,
    _reserved2: [u8; 0x0f78],
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0x80 - Tee mode control register
    #[inline(always)]
    pub const fn m_mode_ctrl(&self, n: usize) -> &M_MODE_CTRL {
        &self.m_mode_ctrl[n]
    }
    ///Iterator for array of:
    ///0x00..0x80 - Tee mode control register
    #[inline(always)]
    pub fn m_mode_ctrl_iter(&self) -> impl Iterator<Item = &M_MODE_CTRL> {
        self.m_mode_ctrl.iter()
    }
    ///0x00 - Tee mode control register
    #[inline(always)]
    pub const fn m0_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(0)
    }
    ///0x04 - Tee mode control register
    #[inline(always)]
    pub const fn m1_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(1)
    }
    ///0x08 - Tee mode control register
    #[inline(always)]
    pub const fn m2_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(2)
    }
    ///0x0c - Tee mode control register
    #[inline(always)]
    pub const fn m3_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(3)
    }
    ///0x10 - Tee mode control register
    #[inline(always)]
    pub const fn m4_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(4)
    }
    ///0x14 - Tee mode control register
    #[inline(always)]
    pub const fn m5_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(5)
    }
    ///0x18 - Tee mode control register
    #[inline(always)]
    pub const fn m6_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(6)
    }
    ///0x1c - Tee mode control register
    #[inline(always)]
    pub const fn m7_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(7)
    }
    ///0x20 - Tee mode control register
    #[inline(always)]
    pub const fn m8_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(8)
    }
    ///0x24 - Tee mode control register
    #[inline(always)]
    pub const fn m9_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(9)
    }
    ///0x28 - Tee mode control register
    #[inline(always)]
    pub const fn m10_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(10)
    }
    ///0x2c - Tee mode control register
    #[inline(always)]
    pub const fn m11_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(11)
    }
    ///0x30 - Tee mode control register
    #[inline(always)]
    pub const fn m12_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(12)
    }
    ///0x34 - Tee mode control register
    #[inline(always)]
    pub const fn m13_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(13)
    }
    ///0x38 - Tee mode control register
    #[inline(always)]
    pub const fn m14_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(14)
    }
    ///0x3c - Tee mode control register
    #[inline(always)]
    pub const fn m15_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(15)
    }
    ///0x40 - Tee mode control register
    #[inline(always)]
    pub const fn m16_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(16)
    }
    ///0x44 - Tee mode control register
    #[inline(always)]
    pub const fn m17_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(17)
    }
    ///0x48 - Tee mode control register
    #[inline(always)]
    pub const fn m18_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(18)
    }
    ///0x4c - Tee mode control register
    #[inline(always)]
    pub const fn m19_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(19)
    }
    ///0x50 - Tee mode control register
    #[inline(always)]
    pub const fn m20_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(20)
    }
    ///0x54 - Tee mode control register
    #[inline(always)]
    pub const fn m21_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(21)
    }
    ///0x58 - Tee mode control register
    #[inline(always)]
    pub const fn m22_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(22)
    }
    ///0x5c - Tee mode control register
    #[inline(always)]
    pub const fn m23_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(23)
    }
    ///0x60 - Tee mode control register
    #[inline(always)]
    pub const fn m24_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(24)
    }
    ///0x64 - Tee mode control register
    #[inline(always)]
    pub const fn m25_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(25)
    }
    ///0x68 - Tee mode control register
    #[inline(always)]
    pub const fn m26_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(26)
    }
    ///0x6c - Tee mode control register
    #[inline(always)]
    pub const fn m27_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(27)
    }
    ///0x70 - Tee mode control register
    #[inline(always)]
    pub const fn m28_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(28)
    }
    ///0x74 - Tee mode control register
    #[inline(always)]
    pub const fn m29_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(29)
    }
    ///0x78 - Tee mode control register
    #[inline(always)]
    pub const fn m30_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(30)
    }
    ///0x7c - Tee mode control register
    #[inline(always)]
    pub const fn m31_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(31)
    }
    ///0x80 - Clock gating register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0xffc - Version register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**M_MODE_CTRL (rw) register accessor: Tee mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`m_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@m_mode_ctrl`] module*/
pub type M_MODE_CTRL = crate::Reg<m_mode_ctrl::M_MODE_CTRL_SPEC>;
///Tee mode control register
pub mod m_mode_ctrl;
/**CLOCK_GATE (rw) register accessor: Clock gating register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///Clock gating register
pub mod clock_gate;
/**DATE (rw) register accessor: Version register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version register
pub mod date;
