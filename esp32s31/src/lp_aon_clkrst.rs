#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lp_aonclkrst_root_clk_conf: LP_AONCLKRST_ROOT_CLK_CONF,
    lp_aonclkrst_lp_clk_po_en: LP_AONCLKRST_LP_CLK_PO_EN,
    lp_aonclkrst_misc: LP_AONCLKRST_MISC,
    lp_aonclkrst_timer: LP_AONCLKRST_TIMER,
    lp_aonclkrst_wdt: LP_AONCLKRST_WDT,
    lp_aonclkrst_clockcali: LP_AONCLKRST_CLOCKCALI,
    lp_aonclkrst_anaperi: LP_AONCLKRST_ANAPERI,
    lp_aonclkrst_huk: LP_AONCLKRST_HUK,
    lp_aonclkrst_fosc_dfreq: LP_AONCLKRST_FOSC_DFREQ,
    lp_aonclkrst_rc32k_dfreq: LP_AONCLKRST_RC32K_DFREQ,
    lp_aonclkrst_sosc_dfreq: LP_AONCLKRST_SOSC_DFREQ,
    lp_aonclkrst_xtal32k: LP_AONCLKRST_XTAL32K,
    lp_aonclkrst_hpcore0_reset_cause: LP_AONCLKRST_HPCORE0_RESET_CAUSE,
    lp_aonclkrst_hpcore1_reset_cause: LP_AONCLKRST_HPCORE1_RESET_CAUSE,
    lp_aonclkrst_lpcore_reset_cause: LP_AONCLKRST_LPCORE_RESET_CAUSE,
    lp_aonclkrst_hpcore0_reset_ctrl: LP_AONCLKRST_HPCORE0_RESET_CTRL,
    lp_aonclkrst_hpcore1_reset_ctrl: LP_AONCLKRST_HPCORE1_RESET_CTRL,
    lp_aonclkrst_touch_aon: LP_AONCLKRST_TOUCH_AON,
    lp_aonclkrst_hp_clk_ctrl: LP_AONCLKRST_HP_CLK_CTRL,
    lp_aonclkrst_cpll_div: LP_AONCLKRST_CPLL_DIV,
    lp_aonclkrst_apll_div: LP_AONCLKRST_APLL_DIV,
    lp_aonclkrst_mspi_div: LP_AONCLKRST_MSPI_DIV,
    lp_aonclkrst_lp_adi: LP_AONCLKRST_LP_ADI,
    lp_aonclkrst_lproot_clk_div: LP_AONCLKRST_LPROOT_CLK_DIV,
    lp_aonclkrst_cpll_cfg: LP_AONCLKRST_CPLL_CFG,
    lp_aonclkrst_apll_sdm: LP_AONCLKRST_APLL_SDM,
    lp_aonclkrst_rtc_sar2_pwdet_cct: LP_AONCLKRST_RTC_SAR2_PWDET_CCT,
    _reserved27: [u8; 0x0390],
    lp_aonclkrst_date: LP_AONCLKRST_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_root_clk_conf(&self) -> &LP_AONCLKRST_ROOT_CLK_CONF {
        &self.lp_aonclkrst_root_clk_conf
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_clk_po_en(&self) -> &LP_AONCLKRST_LP_CLK_PO_EN {
        &self.lp_aonclkrst_lp_clk_po_en
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_misc(&self) -> &LP_AONCLKRST_MISC {
        &self.lp_aonclkrst_misc
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_timer(&self) -> &LP_AONCLKRST_TIMER {
        &self.lp_aonclkrst_timer
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_wdt(&self) -> &LP_AONCLKRST_WDT {
        &self.lp_aonclkrst_wdt
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_clockcali(&self) -> &LP_AONCLKRST_CLOCKCALI {
        &self.lp_aonclkrst_clockcali
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_anaperi(&self) -> &LP_AONCLKRST_ANAPERI {
        &self.lp_aonclkrst_anaperi
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_huk(&self) -> &LP_AONCLKRST_HUK {
        &self.lp_aonclkrst_huk
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_fosc_dfreq(&self) -> &LP_AONCLKRST_FOSC_DFREQ {
        &self.lp_aonclkrst_fosc_dfreq
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_rc32k_dfreq(&self) -> &LP_AONCLKRST_RC32K_DFREQ {
        &self.lp_aonclkrst_rc32k_dfreq
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_sosc_dfreq(&self) -> &LP_AONCLKRST_SOSC_DFREQ {
        &self.lp_aonclkrst_sosc_dfreq
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_xtal32k(&self) -> &LP_AONCLKRST_XTAL32K {
        &self.lp_aonclkrst_xtal32k
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_reset_cause(&self) -> &LP_AONCLKRST_HPCORE0_RESET_CAUSE {
        &self.lp_aonclkrst_hpcore0_reset_cause
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_reset_cause(&self) -> &LP_AONCLKRST_HPCORE1_RESET_CAUSE {
        &self.lp_aonclkrst_hpcore1_reset_cause
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lpcore_reset_cause(&self) -> &LP_AONCLKRST_LPCORE_RESET_CAUSE {
        &self.lp_aonclkrst_lpcore_reset_cause
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_reset_ctrl(&self) -> &LP_AONCLKRST_HPCORE0_RESET_CTRL {
        &self.lp_aonclkrst_hpcore0_reset_ctrl
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_reset_ctrl(&self) -> &LP_AONCLKRST_HPCORE1_RESET_CTRL {
        &self.lp_aonclkrst_hpcore1_reset_ctrl
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_touch_aon(&self) -> &LP_AONCLKRST_TOUCH_AON {
        &self.lp_aonclkrst_touch_aon
    }
    #[doc = "0x48 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_clk_ctrl(&self) -> &LP_AONCLKRST_HP_CLK_CTRL {
        &self.lp_aonclkrst_hp_clk_ctrl
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_cpll_div(&self) -> &LP_AONCLKRST_CPLL_DIV {
        &self.lp_aonclkrst_cpll_div
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_apll_div(&self) -> &LP_AONCLKRST_APLL_DIV {
        &self.lp_aonclkrst_apll_div
    }
    #[doc = "0x54 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_mspi_div(&self) -> &LP_AONCLKRST_MSPI_DIV {
        &self.lp_aonclkrst_mspi_div
    }
    #[doc = "0x58 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_adi(&self) -> &LP_AONCLKRST_LP_ADI {
        &self.lp_aonclkrst_lp_adi
    }
    #[doc = "0x5c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lproot_clk_div(&self) -> &LP_AONCLKRST_LPROOT_CLK_DIV {
        &self.lp_aonclkrst_lproot_clk_div
    }
    #[doc = "0x60 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_cpll_cfg(&self) -> &LP_AONCLKRST_CPLL_CFG {
        &self.lp_aonclkrst_cpll_cfg
    }
    #[doc = "0x64 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_apll_sdm(&self) -> &LP_AONCLKRST_APLL_SDM {
        &self.lp_aonclkrst_apll_sdm
    }
    #[doc = "0x68 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_rtc_sar2_pwdet_cct(&self) -> &LP_AONCLKRST_RTC_SAR2_PWDET_CCT {
        &self.lp_aonclkrst_rtc_sar2_pwdet_cct
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_date(&self) -> &LP_AONCLKRST_DATE {
        &self.lp_aonclkrst_date
    }
}
#[doc = "LP_AONCLKRST_ROOT_CLK_CONF (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_root_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_root_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_root_clk_conf`] module"]
pub type LP_AONCLKRST_ROOT_CLK_CONF =
    crate::Reg<lp_aonclkrst_root_clk_conf::LP_AONCLKRST_ROOT_CLK_CONF_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_root_clk_conf;
#[doc = "LP_AONCLKRST_LP_CLK_PO_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lp_clk_po_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_po_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_clk_po_en`] module"]
pub type LP_AONCLKRST_LP_CLK_PO_EN =
    crate::Reg<lp_aonclkrst_lp_clk_po_en::LP_AONCLKRST_LP_CLK_PO_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_clk_po_en;
#[doc = "LP_AONCLKRST_MISC (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_misc`] module"]
pub type LP_AONCLKRST_MISC = crate::Reg<lp_aonclkrst_misc::LP_AONCLKRST_MISC_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_misc;
#[doc = "LP_AONCLKRST_TIMER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_timer`] module"]
pub type LP_AONCLKRST_TIMER = crate::Reg<lp_aonclkrst_timer::LP_AONCLKRST_TIMER_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_timer;
#[doc = "LP_AONCLKRST_WDT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_wdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_wdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_wdt`] module"]
pub type LP_AONCLKRST_WDT = crate::Reg<lp_aonclkrst_wdt::LP_AONCLKRST_WDT_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_wdt;
#[doc = "LP_AONCLKRST_CLOCKCALI (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_clockcali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_clockcali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_clockcali`] module"]
pub type LP_AONCLKRST_CLOCKCALI = crate::Reg<lp_aonclkrst_clockcali::LP_AONCLKRST_CLOCKCALI_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_clockcali;
#[doc = "LP_AONCLKRST_ANAPERI (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_anaperi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_anaperi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_anaperi`] module"]
pub type LP_AONCLKRST_ANAPERI = crate::Reg<lp_aonclkrst_anaperi::LP_AONCLKRST_ANAPERI_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_anaperi;
#[doc = "LP_AONCLKRST_HUK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_huk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_huk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_huk`] module"]
pub type LP_AONCLKRST_HUK = crate::Reg<lp_aonclkrst_huk::LP_AONCLKRST_HUK_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_huk;
#[doc = "LP_AONCLKRST_FOSC_DFREQ (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_fosc_dfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_fosc_dfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_fosc_dfreq`] module"]
pub type LP_AONCLKRST_FOSC_DFREQ =
    crate::Reg<lp_aonclkrst_fosc_dfreq::LP_AONCLKRST_FOSC_DFREQ_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_fosc_dfreq;
#[doc = "LP_AONCLKRST_RC32K_DFREQ (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_rc32k_dfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_rc32k_dfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_rc32k_dfreq`] module"]
pub type LP_AONCLKRST_RC32K_DFREQ =
    crate::Reg<lp_aonclkrst_rc32k_dfreq::LP_AONCLKRST_RC32K_DFREQ_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_rc32k_dfreq;
#[doc = "LP_AONCLKRST_SOSC_DFREQ (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_sosc_dfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_sosc_dfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_sosc_dfreq`] module"]
pub type LP_AONCLKRST_SOSC_DFREQ =
    crate::Reg<lp_aonclkrst_sosc_dfreq::LP_AONCLKRST_SOSC_DFREQ_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_sosc_dfreq;
#[doc = "LP_AONCLKRST_XTAL32K (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_xtal32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_xtal32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_xtal32k`] module"]
pub type LP_AONCLKRST_XTAL32K = crate::Reg<lp_aonclkrst_xtal32k::LP_AONCLKRST_XTAL32K_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_xtal32k;
#[doc = "LP_AONCLKRST_HPCORE0_RESET_CAUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpcore0_reset_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcore0_reset_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpcore0_reset_cause`] module"]
pub type LP_AONCLKRST_HPCORE0_RESET_CAUSE =
    crate::Reg<lp_aonclkrst_hpcore0_reset_cause::LP_AONCLKRST_HPCORE0_RESET_CAUSE_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpcore0_reset_cause;
#[doc = "LP_AONCLKRST_HPCORE1_RESET_CAUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpcore1_reset_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcore1_reset_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpcore1_reset_cause`] module"]
pub type LP_AONCLKRST_HPCORE1_RESET_CAUSE =
    crate::Reg<lp_aonclkrst_hpcore1_reset_cause::LP_AONCLKRST_HPCORE1_RESET_CAUSE_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpcore1_reset_cause;
#[doc = "LP_AONCLKRST_LPCORE_RESET_CAUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lpcore_reset_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lpcore_reset_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lpcore_reset_cause`] module"]
pub type LP_AONCLKRST_LPCORE_RESET_CAUSE =
    crate::Reg<lp_aonclkrst_lpcore_reset_cause::LP_AONCLKRST_LPCORE_RESET_CAUSE_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lpcore_reset_cause;
#[doc = "LP_AONCLKRST_HPCORE0_RESET_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpcore0_reset_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcore0_reset_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpcore0_reset_ctrl`] module"]
pub type LP_AONCLKRST_HPCORE0_RESET_CTRL =
    crate::Reg<lp_aonclkrst_hpcore0_reset_ctrl::LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpcore0_reset_ctrl;
#[doc = "LP_AONCLKRST_HPCORE1_RESET_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpcore1_reset_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcore1_reset_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpcore1_reset_ctrl`] module"]
pub type LP_AONCLKRST_HPCORE1_RESET_CTRL =
    crate::Reg<lp_aonclkrst_hpcore1_reset_ctrl::LP_AONCLKRST_HPCORE1_RESET_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpcore1_reset_ctrl;
#[doc = "LP_AONCLKRST_TOUCH_AON (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_touch_aon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_touch_aon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_touch_aon`] module"]
pub type LP_AONCLKRST_TOUCH_AON = crate::Reg<lp_aonclkrst_touch_aon::LP_AONCLKRST_TOUCH_AON_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_touch_aon;
#[doc = "LP_AONCLKRST_HP_CLK_CTRL (r) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_clk_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hp_clk_ctrl`] module"]
pub type LP_AONCLKRST_HP_CLK_CTRL =
    crate::Reg<lp_aonclkrst_hp_clk_ctrl::LP_AONCLKRST_HP_CLK_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod lp_aonclkrst_hp_clk_ctrl;
#[doc = "LP_AONCLKRST_CPLL_DIV (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_cpll_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_cpll_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_cpll_div`] module"]
pub type LP_AONCLKRST_CPLL_DIV = crate::Reg<lp_aonclkrst_cpll_div::LP_AONCLKRST_CPLL_DIV_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_cpll_div;
#[doc = "LP_AONCLKRST_APLL_DIV (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_apll_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_apll_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_apll_div`] module"]
pub type LP_AONCLKRST_APLL_DIV = crate::Reg<lp_aonclkrst_apll_div::LP_AONCLKRST_APLL_DIV_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_apll_div;
#[doc = "LP_AONCLKRST_MSPI_DIV (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_mspi_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_mspi_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_mspi_div`] module"]
pub type LP_AONCLKRST_MSPI_DIV = crate::Reg<lp_aonclkrst_mspi_div::LP_AONCLKRST_MSPI_DIV_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_mspi_div;
#[doc = "LP_AONCLKRST_LP_ADI (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lp_adi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_adi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_adi`] module"]
pub type LP_AONCLKRST_LP_ADI = crate::Reg<lp_aonclkrst_lp_adi::LP_AONCLKRST_LP_ADI_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_adi;
#[doc = "LP_AONCLKRST_LPROOT_CLK_DIV (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lproot_clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lproot_clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lproot_clk_div`] module"]
pub type LP_AONCLKRST_LPROOT_CLK_DIV =
    crate::Reg<lp_aonclkrst_lproot_clk_div::LP_AONCLKRST_LPROOT_CLK_DIV_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lproot_clk_div;
#[doc = "LP_AONCLKRST_CPLL_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_cpll_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_cpll_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_cpll_cfg`] module"]
pub type LP_AONCLKRST_CPLL_CFG = crate::Reg<lp_aonclkrst_cpll_cfg::LP_AONCLKRST_CPLL_CFG_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_cpll_cfg;
#[doc = "LP_AONCLKRST_APLL_SDM (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_apll_sdm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_apll_sdm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_apll_sdm`] module"]
pub type LP_AONCLKRST_APLL_SDM = crate::Reg<lp_aonclkrst_apll_sdm::LP_AONCLKRST_APLL_SDM_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_apll_sdm;
#[doc = "LP_AONCLKRST_RTC_SAR2_PWDET_CCT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_rtc_sar2_pwdet_cct::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_rtc_sar2_pwdet_cct::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_rtc_sar2_pwdet_cct`] module"]
pub type LP_AONCLKRST_RTC_SAR2_PWDET_CCT =
    crate::Reg<lp_aonclkrst_rtc_sar2_pwdet_cct::LP_AONCLKRST_RTC_SAR2_PWDET_CCT_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_rtc_sar2_pwdet_cct;
#[doc = "LP_AONCLKRST_DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_date`] module"]
pub type LP_AONCLKRST_DATE = crate::Reg<lp_aonclkrst_date::LP_AONCLKRST_DATE_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_date;
