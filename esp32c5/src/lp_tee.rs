#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    m0_mode_ctrl: M0_MODE_CTRL,
    clock_gate: CLOCK_GATE,
    _reserved2: [u8; 0x88],
    force_acc_hp: FORCE_ACC_HP,
    _reserved3: [u8; 0x68],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - TEE mode control register"]
    #[inline(always)]
    pub const fn m0_mode_ctrl(&self) -> &M0_MODE_CTRL {
        &self.m0_mode_ctrl
    }
    #[doc = "0x04 - Clock gating register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x90 - Force access to hpmem configuration register"]
    #[inline(always)]
    pub const fn force_acc_hp(&self) -> &FORCE_ACC_HP {
        &self.force_acc_hp
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "M0_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_mode_ctrl`] module"]
pub type M0_MODE_CTRL = crate::Reg<m0_mode_ctrl::M0_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m0_mode_ctrl;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
#[doc = "FORCE_ACC_HP (rw) register accessor: Force access to hpmem configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`force_acc_hp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_acc_hp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_acc_hp`] module"]
pub type FORCE_ACC_HP = crate::Reg<force_acc_hp::FORCE_ACC_HP_SPEC>;
#[doc = "Force access to hpmem configuration register"]
pub mod force_acc_hp;
pub use crate::aes::date;
pub use crate::aes::DATE;
