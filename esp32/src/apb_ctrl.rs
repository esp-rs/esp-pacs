#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sysclk_conf: SYSCLK_CONF,
    xtal_tick_conf: XTAL_TICK_CONF,
    pll_tick_conf: PLL_TICK_CONF,
    ck8m_tick_conf: CK8M_TICK_CONF,
    apb_saradc_ctrl: APB_SARADC_CTRL,
    apb_saradc_ctrl2: APB_SARADC_CTRL2,
    apb_saradc_fsm: APB_SARADC_FSM,
    apb_saradc_sar1_patt_tab: [APB_SARADC_SAR1_PATT_TAB; 4],
    apb_saradc_sar2_patt_tab: [APB_SARADC_SAR2_PATT_TAB; 4],
    apll_tick_conf: APLL_TICK_CONF,
    _reserved10: [u8; 0x3c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn sysclk_conf(&self) -> &SYSCLK_CONF {
        &self.sysclk_conf
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn xtal_tick_conf(&self) -> &XTAL_TICK_CONF {
        &self.xtal_tick_conf
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn pll_tick_conf(&self) -> &PLL_TICK_CONF {
        &self.pll_tick_conf
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ck8m_tick_conf(&self) -> &CK8M_TICK_CONF {
        &self.ck8m_tick_conf
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn apb_saradc_ctrl(&self) -> &APB_SARADC_CTRL {
        &self.apb_saradc_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn apb_saradc_ctrl2(&self) -> &APB_SARADC_CTRL2 {
        &self.apb_saradc_ctrl2
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn apb_saradc_fsm(&self) -> &APB_SARADC_FSM {
        &self.apb_saradc_fsm
    }
    #[doc = "0x1c..0x2c - "]
    #[inline(always)]
    pub const fn apb_saradc_sar1_patt_tab(&self, n: usize) -> &APB_SARADC_SAR1_PATT_TAB {
        &self.apb_saradc_sar1_patt_tab[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x2c - "]
    #[inline(always)]
    pub fn apb_saradc_sar1_patt_tab_iter(&self) -> impl Iterator<Item = &APB_SARADC_SAR1_PATT_TAB> {
        self.apb_saradc_sar1_patt_tab.iter()
    }
    #[doc = "0x1c - APB_SARADC_SAR1_PATT_TAB1"]
    #[inline(always)]
    pub const fn apb_saradc_sar1_patt_tab1(&self) -> &APB_SARADC_SAR1_PATT_TAB {
        self.apb_saradc_sar1_patt_tab(0)
    }
    #[doc = "0x20 - APB_SARADC_SAR1_PATT_TAB2"]
    #[inline(always)]
    pub const fn apb_saradc_sar1_patt_tab2(&self) -> &APB_SARADC_SAR1_PATT_TAB {
        self.apb_saradc_sar1_patt_tab(1)
    }
    #[doc = "0x24 - APB_SARADC_SAR1_PATT_TAB3"]
    #[inline(always)]
    pub const fn apb_saradc_sar1_patt_tab3(&self) -> &APB_SARADC_SAR1_PATT_TAB {
        self.apb_saradc_sar1_patt_tab(2)
    }
    #[doc = "0x28 - APB_SARADC_SAR1_PATT_TAB4"]
    #[inline(always)]
    pub const fn apb_saradc_sar1_patt_tab4(&self) -> &APB_SARADC_SAR1_PATT_TAB {
        self.apb_saradc_sar1_patt_tab(3)
    }
    #[doc = "0x2c..0x3c - "]
    #[inline(always)]
    pub const fn apb_saradc_sar2_patt_tab(&self, n: usize) -> &APB_SARADC_SAR2_PATT_TAB {
        &self.apb_saradc_sar2_patt_tab[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c..0x3c - "]
    #[inline(always)]
    pub fn apb_saradc_sar2_patt_tab_iter(&self) -> impl Iterator<Item = &APB_SARADC_SAR2_PATT_TAB> {
        self.apb_saradc_sar2_patt_tab.iter()
    }
    #[doc = "0x2c - APB_SARADC_SAR2_PATT_TAB1"]
    #[inline(always)]
    pub const fn apb_saradc_sar2_patt_tab1(&self) -> &APB_SARADC_SAR2_PATT_TAB {
        self.apb_saradc_sar2_patt_tab(0)
    }
    #[doc = "0x30 - APB_SARADC_SAR2_PATT_TAB2"]
    #[inline(always)]
    pub const fn apb_saradc_sar2_patt_tab2(&self) -> &APB_SARADC_SAR2_PATT_TAB {
        self.apb_saradc_sar2_patt_tab(1)
    }
    #[doc = "0x34 - APB_SARADC_SAR2_PATT_TAB3"]
    #[inline(always)]
    pub const fn apb_saradc_sar2_patt_tab3(&self) -> &APB_SARADC_SAR2_PATT_TAB {
        self.apb_saradc_sar2_patt_tab(2)
    }
    #[doc = "0x38 - APB_SARADC_SAR2_PATT_TAB4"]
    #[inline(always)]
    pub const fn apb_saradc_sar2_patt_tab4(&self) -> &APB_SARADC_SAR2_PATT_TAB {
        self.apb_saradc_sar2_patt_tab(3)
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn apll_tick_conf(&self) -> &APLL_TICK_CONF {
        &self.apll_tick_conf
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "SYSCLK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclk_conf`] module"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = ""]
pub mod sysclk_conf;
#[doc = "XTAL_TICK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal_tick_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_tick_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_tick_conf`] module"]
pub type XTAL_TICK_CONF = crate::Reg<xtal_tick_conf::XTAL_TICK_CONF_SPEC>;
#[doc = ""]
pub mod xtal_tick_conf;
#[doc = "PLL_TICK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_tick_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_tick_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_tick_conf`] module"]
pub type PLL_TICK_CONF = crate::Reg<pll_tick_conf::PLL_TICK_CONF_SPEC>;
#[doc = ""]
pub mod pll_tick_conf;
#[doc = "CK8M_TICK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ck8m_tick_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck8m_tick_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ck8m_tick_conf`] module"]
pub type CK8M_TICK_CONF = crate::Reg<ck8m_tick_conf::CK8M_TICK_CONF_SPEC>;
#[doc = ""]
pub mod ck8m_tick_conf;
#[doc = "APB_SARADC_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_saradc_ctrl`] module"]
pub type APB_SARADC_CTRL = crate::Reg<apb_saradc_ctrl::APB_SARADC_CTRL_SPEC>;
#[doc = ""]
pub mod apb_saradc_ctrl;
#[doc = "APB_SARADC_CTRL2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_saradc_ctrl2`] module"]
pub type APB_SARADC_CTRL2 = crate::Reg<apb_saradc_ctrl2::APB_SARADC_CTRL2_SPEC>;
#[doc = ""]
pub mod apb_saradc_ctrl2;
#[doc = "APB_SARADC_FSM (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_fsm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_fsm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_saradc_fsm`] module"]
pub type APB_SARADC_FSM = crate::Reg<apb_saradc_fsm::APB_SARADC_FSM_SPEC>;
#[doc = ""]
pub mod apb_saradc_fsm;
#[doc = "APB_SARADC_SAR1_PATT_TAB (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_sar1_patt_tab::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_sar1_patt_tab::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_saradc_sar1_patt_tab`] module"]
pub type APB_SARADC_SAR1_PATT_TAB =
    crate::Reg<apb_saradc_sar1_patt_tab::APB_SARADC_SAR1_PATT_TAB_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar1_patt_tab;
#[doc = "APB_SARADC_SAR2_PATT_TAB (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_sar2_patt_tab::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_sar2_patt_tab::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_saradc_sar2_patt_tab`] module"]
pub type APB_SARADC_SAR2_PATT_TAB =
    crate::Reg<apb_saradc_sar2_patt_tab::APB_SARADC_SAR2_PATT_TAB_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar2_patt_tab;
#[doc = "APLL_TICK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apll_tick_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apll_tick_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apll_tick_conf`] module"]
pub type APLL_TICK_CONF = crate::Reg<apll_tick_conf::APLL_TICK_CONF_SPEC>;
#[doc = ""]
pub mod apll_tick_conf;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
