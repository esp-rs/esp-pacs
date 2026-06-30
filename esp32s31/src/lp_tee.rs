#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    m2_mode_ctrl: M2_MODE_CTRL,
    m16_mode_ctrl: M16_MODE_CTRL,
    m17_mode_ctrl: M17_MODE_CTRL,
    m18_mode_ctrl: M18_MODE_CTRL,
    m19_mode_ctrl: M19_MODE_CTRL,
    force_acc_hp: FORCE_ACC_HP,
    lp_gpio_security: LP_GPIO_SECURITY,
    hp_gpio_security_1: HP_GPIO_SECURITY_1,
    hp_gpio_security_2: HP_GPIO_SECURITY_2,
    _reserved9: [u8; 0x03d4],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - TEE mode control register"]
    #[inline(always)]
    pub const fn m2_mode_ctrl(&self) -> &M2_MODE_CTRL {
        &self.m2_mode_ctrl
    }
    #[doc = "0x04 - TEE mode control register"]
    #[inline(always)]
    pub const fn m16_mode_ctrl(&self) -> &M16_MODE_CTRL {
        &self.m16_mode_ctrl
    }
    #[doc = "0x08 - TEE mode control register"]
    #[inline(always)]
    pub const fn m17_mode_ctrl(&self) -> &M17_MODE_CTRL {
        &self.m17_mode_ctrl
    }
    #[doc = "0x0c - TEE mode control register"]
    #[inline(always)]
    pub const fn m18_mode_ctrl(&self) -> &M18_MODE_CTRL {
        &self.m18_mode_ctrl
    }
    #[doc = "0x10 - TEE mode control register"]
    #[inline(always)]
    pub const fn m19_mode_ctrl(&self) -> &M19_MODE_CTRL {
        &self.m19_mode_ctrl
    }
    #[doc = "0x14 - Force access to hpmem configuration register"]
    #[inline(always)]
    pub const fn force_acc_hp(&self) -> &FORCE_ACC_HP {
        &self.force_acc_hp
    }
    #[doc = "0x18 - need des"]
    #[inline(always)]
    pub const fn lp_gpio_security(&self) -> &LP_GPIO_SECURITY {
        &self.lp_gpio_security
    }
    #[doc = "0x1c - need des"]
    #[inline(always)]
    pub const fn hp_gpio_security_1(&self) -> &HP_GPIO_SECURITY_1 {
        &self.hp_gpio_security_1
    }
    #[doc = "0x20 - need des"]
    #[inline(always)]
    pub const fn hp_gpio_security_2(&self) -> &HP_GPIO_SECURITY_2 {
        &self.hp_gpio_security_2
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
#[doc = "M2_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_mode_ctrl`] module"]
pub type M2_MODE_CTRL = crate::Reg<m2_mode_ctrl::M2_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m2_mode_ctrl;
#[doc = "M16_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m16_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m16_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m16_mode_ctrl`] module"]
pub type M16_MODE_CTRL = crate::Reg<m16_mode_ctrl::M16_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m16_mode_ctrl;
#[doc = "M17_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m17_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m17_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m17_mode_ctrl`] module"]
pub type M17_MODE_CTRL = crate::Reg<m17_mode_ctrl::M17_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m17_mode_ctrl;
#[doc = "M18_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m18_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m18_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m18_mode_ctrl`] module"]
pub type M18_MODE_CTRL = crate::Reg<m18_mode_ctrl::M18_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m18_mode_ctrl;
#[doc = "M19_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m19_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m19_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m19_mode_ctrl`] module"]
pub type M19_MODE_CTRL = crate::Reg<m19_mode_ctrl::M19_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m19_mode_ctrl;
#[doc = "FORCE_ACC_HP (rw) register accessor: Force access to hpmem configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`force_acc_hp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_acc_hp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_acc_hp`] module"]
pub type FORCE_ACC_HP = crate::Reg<force_acc_hp::FORCE_ACC_HP_SPEC>;
#[doc = "Force access to hpmem configuration register"]
pub mod force_acc_hp;
#[doc = "LP_GPIO_SECURITY (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_gpio_security::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_gpio_security::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_gpio_security`] module"]
pub type LP_GPIO_SECURITY = crate::Reg<lp_gpio_security::LP_GPIO_SECURITY_SPEC>;
#[doc = "need des"]
pub mod lp_gpio_security;
#[doc = "HP_GPIO_SECURITY_1 (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpio_security_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpio_security_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpio_security_1`] module"]
pub type HP_GPIO_SECURITY_1 = crate::Reg<hp_gpio_security_1::HP_GPIO_SECURITY_1_SPEC>;
#[doc = "need des"]
pub mod hp_gpio_security_1;
#[doc = "HP_GPIO_SECURITY_2 (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpio_security_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpio_security_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpio_security_2`] module"]
pub type HP_GPIO_SECURITY_2 = crate::Reg<hp_gpio_security_2::HP_GPIO_SECURITY_2_SPEC>;
#[doc = "need des"]
pub mod hp_gpio_security_2;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
