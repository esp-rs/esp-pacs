#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    trng_ctrl: TRNG_CTRL,
    efuse_ctrl: EFUSE_CTRL,
    pmu_ctrl: PMU_CTRL,
    clkrst_ctrl: CLKRST_CTRL,
    lp_aon_ctrl_ctrl: LP_AON_CTRL_CTRL,
    lp_timer_ctrl: LP_TIMER_CTRL,
    lp_wdt_ctrl: LP_WDT_CTRL,
    lpperi_ctrl: LPPERI_CTRL,
    lp_ana_peri_ctrl: LP_ANA_PERI_CTRL,
    lp_touch_ctrl: LP_TOUCH_CTRL,
    touch_aon_ctrl: TOUCH_AON_CTRL,
    lp_io_ctrl: LP_IO_CTRL,
    lp_ble_timer_ctrl: LP_BLE_TIMER_CTRL,
    lp_tee_ctrl: LP_TEE_CTRL,
    huk_ctrl: HUK_CTRL,
    _reserved15: [u8; 0xa8],
    lp_gpio_security: LP_GPIO_SECURITY,
    hp_gpio_security_1: HP_GPIO_SECURITY_1,
    hp_gpio_security_2: HP_GPIO_SECURITY_2,
    bus_err_conf: BUS_ERR_CONF,
    _reserved19: [u8; 0x04],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - trng read/write control register"]
    #[inline(always)]
    pub const fn trng_ctrl(&self) -> &TRNG_CTRL {
        &self.trng_ctrl
    }
    #[doc = "0x04 - efuse read/write control register"]
    #[inline(always)]
    pub const fn efuse_ctrl(&self) -> &EFUSE_CTRL {
        &self.efuse_ctrl
    }
    #[doc = "0x08 - pmu read/write control register"]
    #[inline(always)]
    pub const fn pmu_ctrl(&self) -> &PMU_CTRL {
        &self.pmu_ctrl
    }
    #[doc = "0x0c - clkrst read/write control register"]
    #[inline(always)]
    pub const fn clkrst_ctrl(&self) -> &CLKRST_CTRL {
        &self.clkrst_ctrl
    }
    #[doc = "0x10 - lp_aon_ctrl read/write control register"]
    #[inline(always)]
    pub const fn lp_aon_ctrl_ctrl(&self) -> &LP_AON_CTRL_CTRL {
        &self.lp_aon_ctrl_ctrl
    }
    #[doc = "0x14 - lp_timer read/write control register"]
    #[inline(always)]
    pub const fn lp_timer_ctrl(&self) -> &LP_TIMER_CTRL {
        &self.lp_timer_ctrl
    }
    #[doc = "0x18 - lp_wdt read/write control register"]
    #[inline(always)]
    pub const fn lp_wdt_ctrl(&self) -> &LP_WDT_CTRL {
        &self.lp_wdt_ctrl
    }
    #[doc = "0x1c - lpperi read/write control register"]
    #[inline(always)]
    pub const fn lpperi_ctrl(&self) -> &LPPERI_CTRL {
        &self.lpperi_ctrl
    }
    #[doc = "0x20 - lp_ana_peri read/write control register"]
    #[inline(always)]
    pub const fn lp_ana_peri_ctrl(&self) -> &LP_ANA_PERI_CTRL {
        &self.lp_ana_peri_ctrl
    }
    #[doc = "0x24 - lp_touch read/write control register"]
    #[inline(always)]
    pub const fn lp_touch_ctrl(&self) -> &LP_TOUCH_CTRL {
        &self.lp_touch_ctrl
    }
    #[doc = "0x28 - touch_aon read/write control register"]
    #[inline(always)]
    pub const fn touch_aon_ctrl(&self) -> &TOUCH_AON_CTRL {
        &self.touch_aon_ctrl
    }
    #[doc = "0x2c - lp_io read/write control register"]
    #[inline(always)]
    pub const fn lp_io_ctrl(&self) -> &LP_IO_CTRL {
        &self.lp_io_ctrl
    }
    #[doc = "0x30 - lp_ble_timer read/write control register"]
    #[inline(always)]
    pub const fn lp_ble_timer_ctrl(&self) -> &LP_BLE_TIMER_CTRL {
        &self.lp_ble_timer_ctrl
    }
    #[doc = "0x34 - lp_tee read/write control register"]
    #[inline(always)]
    pub const fn lp_tee_ctrl(&self) -> &LP_TEE_CTRL {
        &self.lp_tee_ctrl
    }
    #[doc = "0x38 - lp_tee read/write control register"]
    #[inline(always)]
    pub const fn huk_ctrl(&self) -> &HUK_CTRL {
        &self.huk_ctrl
    }
    #[doc = "0xe4 - need des"]
    #[inline(always)]
    pub const fn lp_gpio_security(&self) -> &LP_GPIO_SECURITY {
        &self.lp_gpio_security
    }
    #[doc = "0xe8 - need des"]
    #[inline(always)]
    pub const fn hp_gpio_security_1(&self) -> &HP_GPIO_SECURITY_1 {
        &self.hp_gpio_security_1
    }
    #[doc = "0xec - need des"]
    #[inline(always)]
    pub const fn hp_gpio_security_2(&self) -> &HP_GPIO_SECURITY_2 {
        &self.hp_gpio_security_2
    }
    #[doc = "0xf0 - Clock gating register"]
    #[inline(always)]
    pub const fn bus_err_conf(&self) -> &BUS_ERR_CONF {
        &self.bus_err_conf
    }
    #[doc = "0xf8 - Clock gating register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "TRNG_CTRL (rw) register accessor: trng read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_ctrl`] module"]
pub type TRNG_CTRL = crate::Reg<trng_ctrl::TRNG_CTRL_SPEC>;
#[doc = "trng read/write control register"]
pub mod trng_ctrl;
#[doc = "EFUSE_CTRL (rw) register accessor: efuse read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_ctrl`] module"]
pub type EFUSE_CTRL = crate::Reg<efuse_ctrl::EFUSE_CTRL_SPEC>;
#[doc = "efuse read/write control register"]
pub mod efuse_ctrl;
#[doc = "PMU_CTRL (rw) register accessor: pmu read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_ctrl`] module"]
pub type PMU_CTRL = crate::Reg<pmu_ctrl::PMU_CTRL_SPEC>;
#[doc = "pmu read/write control register"]
pub mod pmu_ctrl;
#[doc = "CLKRST_CTRL (rw) register accessor: clkrst read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkrst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkrst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkrst_ctrl`] module"]
pub type CLKRST_CTRL = crate::Reg<clkrst_ctrl::CLKRST_CTRL_SPEC>;
#[doc = "clkrst read/write control register"]
pub mod clkrst_ctrl;
#[doc = "LP_AON_CTRL_CTRL (rw) register accessor: lp_aon_ctrl read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aon_ctrl_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aon_ctrl_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aon_ctrl_ctrl`] module"]
pub type LP_AON_CTRL_CTRL = crate::Reg<lp_aon_ctrl_ctrl::LP_AON_CTRL_CTRL_SPEC>;
#[doc = "lp_aon_ctrl read/write control register"]
pub mod lp_aon_ctrl_ctrl;
#[doc = "LP_TIMER_CTRL (rw) register accessor: lp_timer read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_timer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_timer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timer_ctrl`] module"]
pub type LP_TIMER_CTRL = crate::Reg<lp_timer_ctrl::LP_TIMER_CTRL_SPEC>;
#[doc = "lp_timer read/write control register"]
pub mod lp_timer_ctrl;
#[doc = "LP_WDT_CTRL (rw) register accessor: lp_wdt read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_wdt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_wdt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_wdt_ctrl`] module"]
pub type LP_WDT_CTRL = crate::Reg<lp_wdt_ctrl::LP_WDT_CTRL_SPEC>;
#[doc = "lp_wdt read/write control register"]
pub mod lp_wdt_ctrl;
#[doc = "LPPERI_CTRL (rw) register accessor: lpperi read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpperi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpperi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpperi_ctrl`] module"]
pub type LPPERI_CTRL = crate::Reg<lpperi_ctrl::LPPERI_CTRL_SPEC>;
#[doc = "lpperi read/write control register"]
pub mod lpperi_ctrl;
#[doc = "LP_ANA_PERI_CTRL (rw) register accessor: lp_ana_peri read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_peri_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_peri_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_peri_ctrl`] module"]
pub type LP_ANA_PERI_CTRL = crate::Reg<lp_ana_peri_ctrl::LP_ANA_PERI_CTRL_SPEC>;
#[doc = "lp_ana_peri read/write control register"]
pub mod lp_ana_peri_ctrl;
#[doc = "LP_TOUCH_CTRL (rw) register accessor: lp_touch read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_touch_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_touch_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_touch_ctrl`] module"]
pub type LP_TOUCH_CTRL = crate::Reg<lp_touch_ctrl::LP_TOUCH_CTRL_SPEC>;
#[doc = "lp_touch read/write control register"]
pub mod lp_touch_ctrl;
#[doc = "TOUCH_AON_CTRL (rw) register accessor: touch_aon read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_aon_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_aon_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_aon_ctrl`] module"]
pub type TOUCH_AON_CTRL = crate::Reg<touch_aon_ctrl::TOUCH_AON_CTRL_SPEC>;
#[doc = "touch_aon read/write control register"]
pub mod touch_aon_ctrl;
#[doc = "LP_IO_CTRL (rw) register accessor: lp_io read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_io_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_io_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_io_ctrl`] module"]
pub type LP_IO_CTRL = crate::Reg<lp_io_ctrl::LP_IO_CTRL_SPEC>;
#[doc = "lp_io read/write control register"]
pub mod lp_io_ctrl;
#[doc = "LP_BLE_TIMER_CTRL (rw) register accessor: lp_ble_timer read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ble_timer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ble_timer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ble_timer_ctrl`] module"]
pub type LP_BLE_TIMER_CTRL = crate::Reg<lp_ble_timer_ctrl::LP_BLE_TIMER_CTRL_SPEC>;
#[doc = "lp_ble_timer read/write control register"]
pub mod lp_ble_timer_ctrl;
#[doc = "LP_TEE_CTRL (rw) register accessor: lp_tee read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tee_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tee_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tee_ctrl`] module"]
pub type LP_TEE_CTRL = crate::Reg<lp_tee_ctrl::LP_TEE_CTRL_SPEC>;
#[doc = "lp_tee read/write control register"]
pub mod lp_tee_ctrl;
#[doc = "HUK_CTRL (rw) register accessor: lp_tee read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`huk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@huk_ctrl`] module"]
pub type HUK_CTRL = crate::Reg<huk_ctrl::HUK_CTRL_SPEC>;
#[doc = "lp_tee read/write control register"]
pub mod huk_ctrl;
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
#[doc = "BUS_ERR_CONF (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_err_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_err_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_err_conf`] module"]
pub type BUS_ERR_CONF = crate::Reg<bus_err_conf::BUS_ERR_CONF_SPEC>;
#[doc = "Clock gating register"]
pub mod bus_err_conf;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
pub use crate::dma::{date, DATE};
