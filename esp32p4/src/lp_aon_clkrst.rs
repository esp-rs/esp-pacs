#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lp_aonclkrst_lp_clk_conf: LP_AONCLKRST_LP_CLK_CONF,
    lp_aonclkrst_lp_clk_po_en: LP_AONCLKRST_LP_CLK_PO_EN,
    lp_aonclkrst_lp_clk_en: LP_AONCLKRST_LP_CLK_EN,
    lp_aonclkrst_lp_rst_en: LP_AONCLKRST_LP_RST_EN,
    lp_aonclkrst_reset_cause: LP_AONCLKRST_RESET_CAUSE,
    lp_aonclkrst_hpcpu_reset_ctrl0: LP_AONCLKRST_HPCPU_RESET_CTRL0,
    lp_aonclkrst_hpcpu_reset_ctrl1: LP_AONCLKRST_HPCPU_RESET_CTRL1,
    lp_aonclkrst_fosc_cntl: LP_AONCLKRST_FOSC_CNTL,
    lp_aonclkrst_rc32k_cntl: LP_AONCLKRST_RC32K_CNTL,
    lp_aonclkrst_sosc_cntl: LP_AONCLKRST_SOSC_CNTL,
    lp_aonclkrst_clk_to_hp: LP_AONCLKRST_CLK_TO_HP,
    lp_aonclkrst_lpmem_force: LP_AONCLKRST_LPMEM_FORCE,
    lp_aonclkrst_xtal32k: LP_AONCLKRST_XTAL32K,
    lp_aonclkrst_mux_hpsys_reset_bypass: LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS,
    lp_aonclkrst_hpsys_0_reset_bypass: LP_AONCLKRST_HPSYS_0_RESET_BYPASS,
    lp_aonclkrst_hpsys_apm_reset_bypass: LP_AONCLKRST_HPSYS_APM_RESET_BYPASS,
    lp_aonclkrst_hp_clk_ctrl: LP_AONCLKRST_HP_CLK_CTRL,
    lp_aonclkrst_hp_usb_clkrst_ctrl0: LP_AONCLKRST_HP_USB_CLKRST_CTRL0,
    lp_aonclkrst_hp_usb_clkrst_ctrl1: LP_AONCLKRST_HP_USB_CLKRST_CTRL1,
    lp_aonclkrst_hp_sdmmc_emac_rst_ctrl: LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL,
    _reserved20: [u8; 0x03ac],
    lp_aonclkrst_date: LP_AONCLKRST_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_clk_conf(&self) -> &LP_AONCLKRST_LP_CLK_CONF {
        &self.lp_aonclkrst_lp_clk_conf
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_clk_po_en(&self) -> &LP_AONCLKRST_LP_CLK_PO_EN {
        &self.lp_aonclkrst_lp_clk_po_en
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_clk_en(&self) -> &LP_AONCLKRST_LP_CLK_EN {
        &self.lp_aonclkrst_lp_clk_en
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_rst_en(&self) -> &LP_AONCLKRST_LP_RST_EN {
        &self.lp_aonclkrst_lp_rst_en
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_reset_cause(&self) -> &LP_AONCLKRST_RESET_CAUSE {
        &self.lp_aonclkrst_reset_cause
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcpu_reset_ctrl0(&self) -> &LP_AONCLKRST_HPCPU_RESET_CTRL0 {
        &self.lp_aonclkrst_hpcpu_reset_ctrl0
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcpu_reset_ctrl1(&self) -> &LP_AONCLKRST_HPCPU_RESET_CTRL1 {
        &self.lp_aonclkrst_hpcpu_reset_ctrl1
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_fosc_cntl(&self) -> &LP_AONCLKRST_FOSC_CNTL {
        &self.lp_aonclkrst_fosc_cntl
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_rc32k_cntl(&self) -> &LP_AONCLKRST_RC32K_CNTL {
        &self.lp_aonclkrst_rc32k_cntl
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_sosc_cntl(&self) -> &LP_AONCLKRST_SOSC_CNTL {
        &self.lp_aonclkrst_sosc_cntl
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_to_hp(&self) -> &LP_AONCLKRST_CLK_TO_HP {
        &self.lp_aonclkrst_clk_to_hp
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lpmem_force(&self) -> &LP_AONCLKRST_LPMEM_FORCE {
        &self.lp_aonclkrst_lpmem_force
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_xtal32k(&self) -> &LP_AONCLKRST_XTAL32K {
        &self.lp_aonclkrst_xtal32k
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_mux_hpsys_reset_bypass(
        &self,
    ) -> &LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS {
        &self.lp_aonclkrst_mux_hpsys_reset_bypass
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpsys_0_reset_bypass(&self) -> &LP_AONCLKRST_HPSYS_0_RESET_BYPASS {
        &self.lp_aonclkrst_hpsys_0_reset_bypass
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpsys_apm_reset_bypass(
        &self,
    ) -> &LP_AONCLKRST_HPSYS_APM_RESET_BYPASS {
        &self.lp_aonclkrst_hpsys_apm_reset_bypass
    }
    #[doc = "0x40 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_clk_ctrl(&self) -> &LP_AONCLKRST_HP_CLK_CTRL {
        &self.lp_aonclkrst_hp_clk_ctrl
    }
    #[doc = "0x44 - HP USB Clock Reset Control Register."]
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_usb_clkrst_ctrl0(&self) -> &LP_AONCLKRST_HP_USB_CLKRST_CTRL0 {
        &self.lp_aonclkrst_hp_usb_clkrst_ctrl0
    }
    #[doc = "0x48 - HP USB Clock Reset Control Register."]
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_usb_clkrst_ctrl1(&self) -> &LP_AONCLKRST_HP_USB_CLKRST_CTRL1 {
        &self.lp_aonclkrst_hp_usb_clkrst_ctrl1
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_sdmmc_emac_rst_ctrl(
        &self,
    ) -> &LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL {
        &self.lp_aonclkrst_hp_sdmmc_emac_rst_ctrl
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_date(&self) -> &LP_AONCLKRST_DATE {
        &self.lp_aonclkrst_date
    }
}
#[doc = "LP_AONCLKRST_LP_CLK_CONF (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_lp_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_clk_conf`] module"]
pub type LP_AONCLKRST_LP_CLK_CONF =
    crate::Reg<lp_aonclkrst_lp_clk_conf::LP_AONCLKRST_LP_CLK_CONF_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_clk_conf;
#[doc = "LP_AONCLKRST_LP_CLK_PO_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_lp_clk_po_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_po_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_clk_po_en`] module"]
pub type LP_AONCLKRST_LP_CLK_PO_EN =
    crate::Reg<lp_aonclkrst_lp_clk_po_en::LP_AONCLKRST_LP_CLK_PO_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_clk_po_en;
#[doc = "LP_AONCLKRST_LP_CLK_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_lp_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_clk_en`] module"]
pub type LP_AONCLKRST_LP_CLK_EN = crate::Reg<lp_aonclkrst_lp_clk_en::LP_AONCLKRST_LP_CLK_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_clk_en;
#[doc = "LP_AONCLKRST_LP_RST_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_lp_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_rst_en`] module"]
pub type LP_AONCLKRST_LP_RST_EN = crate::Reg<lp_aonclkrst_lp_rst_en::LP_AONCLKRST_LP_RST_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_rst_en;
#[doc = "LP_AONCLKRST_RESET_CAUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_reset_cause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_reset_cause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_reset_cause`] module"]
pub type LP_AONCLKRST_RESET_CAUSE =
    crate::Reg<lp_aonclkrst_reset_cause::LP_AONCLKRST_RESET_CAUSE_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_reset_cause;
#[doc = "LP_AONCLKRST_HPCPU_RESET_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hpcpu_reset_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcpu_reset_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpcpu_reset_ctrl0`] module"]
pub type LP_AONCLKRST_HPCPU_RESET_CTRL0 =
    crate::Reg<lp_aonclkrst_hpcpu_reset_ctrl0::LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpcpu_reset_ctrl0;
#[doc = "LP_AONCLKRST_HPCPU_RESET_CTRL1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hpcpu_reset_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcpu_reset_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpcpu_reset_ctrl1`] module"]
pub type LP_AONCLKRST_HPCPU_RESET_CTRL1 =
    crate::Reg<lp_aonclkrst_hpcpu_reset_ctrl1::LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpcpu_reset_ctrl1;
#[doc = "LP_AONCLKRST_FOSC_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_fosc_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_fosc_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_fosc_cntl`] module"]
pub type LP_AONCLKRST_FOSC_CNTL = crate::Reg<lp_aonclkrst_fosc_cntl::LP_AONCLKRST_FOSC_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_fosc_cntl;
#[doc = "LP_AONCLKRST_RC32K_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_rc32k_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_rc32k_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_rc32k_cntl`] module"]
pub type LP_AONCLKRST_RC32K_CNTL =
    crate::Reg<lp_aonclkrst_rc32k_cntl::LP_AONCLKRST_RC32K_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_rc32k_cntl;
#[doc = "LP_AONCLKRST_SOSC_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_sosc_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_sosc_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_sosc_cntl`] module"]
pub type LP_AONCLKRST_SOSC_CNTL = crate::Reg<lp_aonclkrst_sosc_cntl::LP_AONCLKRST_SOSC_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_sosc_cntl;
#[doc = "LP_AONCLKRST_CLK_TO_HP (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_clk_to_hp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_clk_to_hp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_clk_to_hp`] module"]
pub type LP_AONCLKRST_CLK_TO_HP = crate::Reg<lp_aonclkrst_clk_to_hp::LP_AONCLKRST_CLK_TO_HP_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_clk_to_hp;
#[doc = "LP_AONCLKRST_LPMEM_FORCE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_lpmem_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_lpmem_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lpmem_force`] module"]
pub type LP_AONCLKRST_LPMEM_FORCE =
    crate::Reg<lp_aonclkrst_lpmem_force::LP_AONCLKRST_LPMEM_FORCE_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lpmem_force;
#[doc = "LP_AONCLKRST_XTAL32K (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_xtal32k::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_xtal32k::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_xtal32k`] module"]
pub type LP_AONCLKRST_XTAL32K = crate::Reg<lp_aonclkrst_xtal32k::LP_AONCLKRST_XTAL32K_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_xtal32k;
#[doc = "LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_mux_hpsys_reset_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_mux_hpsys_reset_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_mux_hpsys_reset_bypass`] module"]
pub type LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS =
    crate::Reg<lp_aonclkrst_mux_hpsys_reset_bypass::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_mux_hpsys_reset_bypass;
#[doc = "LP_AONCLKRST_HPSYS_0_RESET_BYPASS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hpsys_0_reset_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hpsys_0_reset_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpsys_0_reset_bypass`] module"]
pub type LP_AONCLKRST_HPSYS_0_RESET_BYPASS =
    crate::Reg<lp_aonclkrst_hpsys_0_reset_bypass::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpsys_0_reset_bypass;
#[doc = "LP_AONCLKRST_HPSYS_APM_RESET_BYPASS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hpsys_apm_reset_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hpsys_apm_reset_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpsys_apm_reset_bypass`] module"]
pub type LP_AONCLKRST_HPSYS_APM_RESET_BYPASS =
    crate::Reg<lp_aonclkrst_hpsys_apm_reset_bypass::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpsys_apm_reset_bypass;
#[doc = "LP_AONCLKRST_HP_CLK_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hp_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hp_clk_ctrl`] module"]
pub type LP_AONCLKRST_HP_CLK_CTRL =
    crate::Reg<lp_aonclkrst_hp_clk_ctrl::LP_AONCLKRST_HP_CLK_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod lp_aonclkrst_hp_clk_ctrl;
#[doc = "LP_AONCLKRST_HP_USB_CLKRST_CTRL0 (rw) register accessor: HP USB Clock Reset Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hp_usb_clkrst_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_usb_clkrst_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hp_usb_clkrst_ctrl0`] module"]
pub type LP_AONCLKRST_HP_USB_CLKRST_CTRL0 =
    crate::Reg<lp_aonclkrst_hp_usb_clkrst_ctrl0::LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC>;
#[doc = "HP USB Clock Reset Control Register."]
pub mod lp_aonclkrst_hp_usb_clkrst_ctrl0;
#[doc = "LP_AONCLKRST_HP_USB_CLKRST_CTRL1 (rw) register accessor: HP USB Clock Reset Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hp_usb_clkrst_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_usb_clkrst_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hp_usb_clkrst_ctrl1`] module"]
pub type LP_AONCLKRST_HP_USB_CLKRST_CTRL1 =
    crate::Reg<lp_aonclkrst_hp_usb_clkrst_ctrl1::LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC>;
#[doc = "HP USB Clock Reset Control Register."]
pub mod lp_aonclkrst_hp_usb_clkrst_ctrl1;
#[doc = "LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hp_sdmmc_emac_rst_ctrl`] module"]
pub type LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL =
    crate::Reg<lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hp_sdmmc_emac_rst_ctrl;
#[doc = "LP_AONCLKRST_DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_date`] module"]
pub type LP_AONCLKRST_DATE = crate::Reg<lp_aonclkrst_date::LP_AONCLKRST_DATE_SPEC>;
#[doc = "need_des"]
pub mod lp_aonclkrst_date;
