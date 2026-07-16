#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    root_clk_conf: ROOT_CLK_CONF,
    lp_clk_po_en: LP_CLK_PO_EN,
    misc: MISC,
    timer: TIMER,
    wdt: WDT,
    clockcali: CLOCKCALI,
    anaperi: ANAPERI,
    huk: HUK,
    fosc_dfreq: FOSC_DFREQ,
    rc32k_dfreq: RC32K_DFREQ,
    sosc_dfreq: SOSC_DFREQ,
    xtal32k: XTAL32K,
    hpcore0_reset_cause: HPCORE0_RESET_CAUSE,
    hpcore1_reset_cause: HPCORE1_RESET_CAUSE,
    lpcore_reset_cause: LPCORE_RESET_CAUSE,
    hpcore0_reset_ctrl: HPCORE0_RESET_CTRL,
    hpcore1_reset_ctrl: HPCORE1_RESET_CTRL,
    touch_aon: TOUCH_AON,
    hp_clk_ctrl: HP_CLK_CTRL,
    cpll_div: CPLL_DIV,
    apll_div: APLL_DIV,
    mspi_div: MSPI_DIV,
    lp_adi: LP_ADI,
    lproot_clk_div: LPROOT_CLK_DIV,
    cpll_cfg: CPLL_CFG,
    apll_sdm: APLL_SDM,
    rtc_sar2_pwdet_cct: RTC_SAR2_PWDET_CCT,
    _reserved27: [u8; 0x0390],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn root_clk_conf(&self) -> &ROOT_CLK_CONF {
        &self.root_clk_conf
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn lp_clk_po_en(&self) -> &LP_CLK_PO_EN {
        &self.lp_clk_po_en
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn timer(&self) -> &TIMER {
        &self.timer
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn wdt(&self) -> &WDT {
        &self.wdt
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn clockcali(&self) -> &CLOCKCALI {
        &self.clockcali
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn anaperi(&self) -> &ANAPERI {
        &self.anaperi
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn huk(&self) -> &HUK {
        &self.huk
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn fosc_dfreq(&self) -> &FOSC_DFREQ {
        &self.fosc_dfreq
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn rc32k_dfreq(&self) -> &RC32K_DFREQ {
        &self.rc32k_dfreq
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn sosc_dfreq(&self) -> &SOSC_DFREQ {
        &self.sosc_dfreq
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn xtal32k(&self) -> &XTAL32K {
        &self.xtal32k
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn hpcore0_reset_cause(&self) -> &HPCORE0_RESET_CAUSE {
        &self.hpcore0_reset_cause
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn hpcore1_reset_cause(&self) -> &HPCORE1_RESET_CAUSE {
        &self.hpcore1_reset_cause
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lpcore_reset_cause(&self) -> &LPCORE_RESET_CAUSE {
        &self.lpcore_reset_cause
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn hpcore0_reset_ctrl(&self) -> &HPCORE0_RESET_CTRL {
        &self.hpcore0_reset_ctrl
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn hpcore1_reset_ctrl(&self) -> &HPCORE1_RESET_CTRL {
        &self.hpcore1_reset_ctrl
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn touch_aon(&self) -> &TOUCH_AON {
        &self.touch_aon
    }
    #[doc = "0x48 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn hp_clk_ctrl(&self) -> &HP_CLK_CTRL {
        &self.hp_clk_ctrl
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn cpll_div(&self) -> &CPLL_DIV {
        &self.cpll_div
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn apll_div(&self) -> &APLL_DIV {
        &self.apll_div
    }
    #[doc = "0x54 - need_des"]
    #[inline(always)]
    pub const fn mspi_div(&self) -> &MSPI_DIV {
        &self.mspi_div
    }
    #[doc = "0x58 - need_des"]
    #[inline(always)]
    pub const fn lp_adi(&self) -> &LP_ADI {
        &self.lp_adi
    }
    #[doc = "0x5c - need_des"]
    #[inline(always)]
    pub const fn lproot_clk_div(&self) -> &LPROOT_CLK_DIV {
        &self.lproot_clk_div
    }
    #[doc = "0x60 - need_des"]
    #[inline(always)]
    pub const fn cpll_cfg(&self) -> &CPLL_CFG {
        &self.cpll_cfg
    }
    #[doc = "0x64 - need_des"]
    #[inline(always)]
    pub const fn apll_sdm(&self) -> &APLL_SDM {
        &self.apll_sdm
    }
    #[doc = "0x68 - need_des"]
    #[inline(always)]
    pub const fn rtc_sar2_pwdet_cct(&self) -> &RTC_SAR2_PWDET_CCT {
        &self.rtc_sar2_pwdet_cct
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "ROOT_CLK_CONF (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_clk_conf`] module"]
pub type ROOT_CLK_CONF = crate::Reg<root_clk_conf::ROOT_CLK_CONF_SPEC>;
#[doc = "need_des"]
pub mod root_clk_conf;
#[doc = "LP_CLK_PO_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_po_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_po_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_po_en`] module"]
pub type LP_CLK_PO_EN = crate::Reg<lp_clk_po_en::LP_CLK_PO_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_clk_po_en;
#[doc = "MISC (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "need_des"]
pub mod misc;
#[doc = "TIMER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer`] module"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "need_des"]
pub mod timer;
#[doc = "WDT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt`] module"]
pub type WDT = crate::Reg<wdt::WDT_SPEC>;
#[doc = "need_des"]
pub mod wdt;
#[doc = "CLOCKCALI (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clockcali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockcali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clockcali`] module"]
pub type CLOCKCALI = crate::Reg<clockcali::CLOCKCALI_SPEC>;
#[doc = "need_des"]
pub mod clockcali;
#[doc = "ANAPERI (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`anaperi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anaperi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anaperi`] module"]
pub type ANAPERI = crate::Reg<anaperi::ANAPERI_SPEC>;
#[doc = "need_des"]
pub mod anaperi;
#[doc = "HUK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`huk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@huk`] module"]
pub type HUK = crate::Reg<huk::HUK_SPEC>;
#[doc = "need_des"]
pub mod huk;
#[doc = "FOSC_DFREQ (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`fosc_dfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fosc_dfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fosc_dfreq`] module"]
pub type FOSC_DFREQ = crate::Reg<fosc_dfreq::FOSC_DFREQ_SPEC>;
#[doc = "need_des"]
pub mod fosc_dfreq;
#[doc = "RC32K_DFREQ (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32k_dfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32k_dfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32k_dfreq`] module"]
pub type RC32K_DFREQ = crate::Reg<rc32k_dfreq::RC32K_DFREQ_SPEC>;
#[doc = "need_des"]
pub mod rc32k_dfreq;
#[doc = "SOSC_DFREQ (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sosc_dfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosc_dfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sosc_dfreq`] module"]
pub type SOSC_DFREQ = crate::Reg<sosc_dfreq::SOSC_DFREQ_SPEC>;
#[doc = "need_des"]
pub mod sosc_dfreq;
#[doc = "XTAL32K (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32k`] module"]
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
#[doc = "need_des"]
pub mod xtal32k;
#[doc = "HPCORE0_RESET_CAUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore0_reset_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore0_reset_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore0_reset_cause`] module"]
pub type HPCORE0_RESET_CAUSE = crate::Reg<hpcore0_reset_cause::HPCORE0_RESET_CAUSE_SPEC>;
#[doc = "need_des"]
pub mod hpcore0_reset_cause;
#[doc = "HPCORE1_RESET_CAUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore1_reset_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore1_reset_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore1_reset_cause`] module"]
pub type HPCORE1_RESET_CAUSE = crate::Reg<hpcore1_reset_cause::HPCORE1_RESET_CAUSE_SPEC>;
#[doc = "need_des"]
pub mod hpcore1_reset_cause;
#[doc = "LPCORE_RESET_CAUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcore_reset_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcore_reset_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcore_reset_cause`] module"]
pub type LPCORE_RESET_CAUSE = crate::Reg<lpcore_reset_cause::LPCORE_RESET_CAUSE_SPEC>;
#[doc = "need_des"]
pub mod lpcore_reset_cause;
#[doc = "HPCORE0_RESET_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore0_reset_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore0_reset_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore0_reset_ctrl`] module"]
pub type HPCORE0_RESET_CTRL = crate::Reg<hpcore0_reset_ctrl::HPCORE0_RESET_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hpcore0_reset_ctrl;
#[doc = "HPCORE1_RESET_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore1_reset_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore1_reset_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore1_reset_ctrl`] module"]
pub type HPCORE1_RESET_CTRL = crate::Reg<hpcore1_reset_ctrl::HPCORE1_RESET_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hpcore1_reset_ctrl;
#[doc = "TOUCH_AON (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_aon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_aon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_aon`] module"]
pub type TOUCH_AON = crate::Reg<touch_aon::TOUCH_AON_SPEC>;
#[doc = "need_des"]
pub mod touch_aon;
#[doc = "HP_CLK_CTRL (r) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_clk_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_clk_ctrl`] module"]
pub type HP_CLK_CTRL = crate::Reg<hp_clk_ctrl::HP_CLK_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod hp_clk_ctrl;
#[doc = "CPLL_DIV (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpll_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpll_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpll_div`] module"]
pub type CPLL_DIV = crate::Reg<cpll_div::CPLL_DIV_SPEC>;
#[doc = "need_des"]
pub mod cpll_div;
#[doc = "APLL_DIV (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`apll_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apll_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apll_div`] module"]
pub type APLL_DIV = crate::Reg<apll_div::APLL_DIV_SPEC>;
#[doc = "need_des"]
pub mod apll_div;
#[doc = "MSPI_DIV (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_div`] module"]
pub type MSPI_DIV = crate::Reg<mspi_div::MSPI_DIV_SPEC>;
#[doc = "need_des"]
pub mod mspi_div;
#[doc = "LP_ADI (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_adi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_adi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_adi`] module"]
pub type LP_ADI = crate::Reg<lp_adi::LP_ADI_SPEC>;
#[doc = "need_des"]
pub mod lp_adi;
#[doc = "LPROOT_CLK_DIV (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lproot_clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lproot_clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lproot_clk_div`] module"]
pub type LPROOT_CLK_DIV = crate::Reg<lproot_clk_div::LPROOT_CLK_DIV_SPEC>;
#[doc = "need_des"]
pub mod lproot_clk_div;
#[doc = "CPLL_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpll_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpll_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpll_cfg`] module"]
pub type CPLL_CFG = crate::Reg<cpll_cfg::CPLL_CFG_SPEC>;
#[doc = "need_des"]
pub mod cpll_cfg;
#[doc = "APLL_SDM (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`apll_sdm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apll_sdm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apll_sdm`] module"]
pub type APLL_SDM = crate::Reg<apll_sdm::APLL_SDM_SPEC>;
#[doc = "need_des"]
pub mod apll_sdm;
#[doc = "RTC_SAR2_PWDET_CCT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_sar2_pwdet_cct::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_sar2_pwdet_cct::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_sar2_pwdet_cct`] module"]
pub type RTC_SAR2_PWDET_CCT = crate::Reg<rtc_sar2_pwdet_cct::RTC_SAR2_PWDET_CCT_SPEC>;
#[doc = "need_des"]
pub mod rtc_sar2_pwdet_cct;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
