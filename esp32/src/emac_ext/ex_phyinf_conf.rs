#[doc = "Register `EX_PHYINF_CONF` reader"]
pub type R = crate::R<EX_PHYINF_CONF_SPEC>;
#[doc = "Register `EX_PHYINF_CONF` writer"]
pub type W = crate::W<EX_PHYINF_CONF_SPEC>;
#[doc = "Field `INT_REVMII_RX_CLK_SEL` reader - "]
pub type INT_REVMII_RX_CLK_SEL_R = crate::BitReader;
#[doc = "Field `INT_REVMII_RX_CLK_SEL` writer - "]
pub type INT_REVMII_RX_CLK_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXT_REVMII_RX_CLK_SEL` reader - "]
pub type EXT_REVMII_RX_CLK_SEL_R = crate::BitReader;
#[doc = "Field `EXT_REVMII_RX_CLK_SEL` writer - "]
pub type EXT_REVMII_RX_CLK_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SBD_FLOWCTRL` reader - "]
pub type SBD_FLOWCTRL_R = crate::BitReader;
#[doc = "Field `SBD_FLOWCTRL` writer - "]
pub type SBD_FLOWCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE_PHY_ADDR` reader - "]
pub type CORE_PHY_ADDR_R = crate::FieldReader;
#[doc = "Field `CORE_PHY_ADDR` writer - "]
pub type CORE_PHY_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `REVMII_PHY_ADDR` reader - "]
pub type REVMII_PHY_ADDR_R = crate::FieldReader;
#[doc = "Field `REVMII_PHY_ADDR` writer - "]
pub type REVMII_PHY_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PHY_INTF_SEL` reader - "]
pub type PHY_INTF_SEL_R = crate::FieldReader;
#[doc = "Field `PHY_INTF_SEL` writer - "]
pub type PHY_INTF_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SS_MODE` reader - "]
pub type SS_MODE_R = crate::BitReader;
#[doc = "Field `SS_MODE` writer - "]
pub type SS_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SBD_CLK_GATING_EN` reader - "]
pub type SBD_CLK_GATING_EN_R = crate::BitReader;
#[doc = "Field `SBD_CLK_GATING_EN` writer - "]
pub type SBD_CLK_GATING_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PMT_CTRL_EN` reader - "]
pub type PMT_CTRL_EN_R = crate::BitReader;
#[doc = "Field `PMT_CTRL_EN` writer - "]
pub type PMT_CTRL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCR_SMI_DLY_RX_SYNC` reader - "]
pub type SCR_SMI_DLY_RX_SYNC_R = crate::BitReader;
#[doc = "Field `SCR_SMI_DLY_RX_SYNC` writer - "]
pub type SCR_SMI_DLY_RX_SYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_ERR_OUT_EN` reader - "]
pub type TX_ERR_OUT_EN_R = crate::BitReader;
#[doc = "Field `TX_ERR_OUT_EN` writer - "]
pub type TX_ERR_OUT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int_revmii_rx_clk_sel(&self) -> INT_REVMII_RX_CLK_SEL_R {
        INT_REVMII_RX_CLK_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ext_revmii_rx_clk_sel(&self) -> EXT_REVMII_RX_CLK_SEL_R {
        EXT_REVMII_RX_CLK_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sbd_flowctrl(&self) -> SBD_FLOWCTRL_R {
        SBD_FLOWCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn core_phy_addr(&self) -> CORE_PHY_ADDR_R {
        CORE_PHY_ADDR_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn revmii_phy_addr(&self) -> REVMII_PHY_ADDR_R {
        REVMII_PHY_ADDR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn phy_intf_sel(&self) -> PHY_INTF_SEL_R {
        PHY_INTF_SEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ss_mode(&self) -> SS_MODE_R {
        SS_MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sbd_clk_gating_en(&self) -> SBD_CLK_GATING_EN_R {
        SBD_CLK_GATING_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pmt_ctrl_en(&self) -> PMT_CTRL_EN_R {
        PMT_CTRL_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn scr_smi_dly_rx_sync(&self) -> SCR_SMI_DLY_RX_SYNC_R {
        SCR_SMI_DLY_RX_SYNC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_err_out_en(&self) -> TX_ERR_OUT_EN_R {
        TX_ERR_OUT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EX_PHYINF_CONF")
            .field(
                "int_revmii_rx_clk_sel",
                &format_args!("{}", self.int_revmii_rx_clk_sel().bit()),
            )
            .field(
                "ext_revmii_rx_clk_sel",
                &format_args!("{}", self.ext_revmii_rx_clk_sel().bit()),
            )
            .field(
                "sbd_flowctrl",
                &format_args!("{}", self.sbd_flowctrl().bit()),
            )
            .field(
                "core_phy_addr",
                &format_args!("{}", self.core_phy_addr().bits()),
            )
            .field(
                "revmii_phy_addr",
                &format_args!("{}", self.revmii_phy_addr().bits()),
            )
            .field(
                "phy_intf_sel",
                &format_args!("{}", self.phy_intf_sel().bits()),
            )
            .field("ss_mode", &format_args!("{}", self.ss_mode().bit()))
            .field(
                "sbd_clk_gating_en",
                &format_args!("{}", self.sbd_clk_gating_en().bit()),
            )
            .field("pmt_ctrl_en", &format_args!("{}", self.pmt_ctrl_en().bit()))
            .field(
                "scr_smi_dly_rx_sync",
                &format_args!("{}", self.scr_smi_dly_rx_sync().bit()),
            )
            .field(
                "tx_err_out_en",
                &format_args!("{}", self.tx_err_out_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EX_PHYINF_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn int_revmii_rx_clk_sel(&mut self) -> INT_REVMII_RX_CLK_SEL_W<EX_PHYINF_CONF_SPEC, 0> {
        INT_REVMII_RX_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ext_revmii_rx_clk_sel(&mut self) -> EXT_REVMII_RX_CLK_SEL_W<EX_PHYINF_CONF_SPEC, 1> {
        EXT_REVMII_RX_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sbd_flowctrl(&mut self) -> SBD_FLOWCTRL_W<EX_PHYINF_CONF_SPEC, 2> {
        SBD_FLOWCTRL_W::new(self)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    #[must_use]
    pub fn core_phy_addr(&mut self) -> CORE_PHY_ADDR_W<EX_PHYINF_CONF_SPEC, 3> {
        CORE_PHY_ADDR_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn revmii_phy_addr(&mut self) -> REVMII_PHY_ADDR_W<EX_PHYINF_CONF_SPEC, 8> {
        REVMII_PHY_ADDR_W::new(self)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    #[must_use]
    pub fn phy_intf_sel(&mut self) -> PHY_INTF_SEL_W<EX_PHYINF_CONF_SPEC, 13> {
        PHY_INTF_SEL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ss_mode(&mut self) -> SS_MODE_W<EX_PHYINF_CONF_SPEC, 16> {
        SS_MODE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn sbd_clk_gating_en(&mut self) -> SBD_CLK_GATING_EN_W<EX_PHYINF_CONF_SPEC, 17> {
        SBD_CLK_GATING_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pmt_ctrl_en(&mut self) -> PMT_CTRL_EN_W<EX_PHYINF_CONF_SPEC, 18> {
        PMT_CTRL_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn scr_smi_dly_rx_sync(&mut self) -> SCR_SMI_DLY_RX_SYNC_W<EX_PHYINF_CONF_SPEC, 19> {
        SCR_SMI_DLY_RX_SYNC_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn tx_err_out_en(&mut self) -> TX_ERR_OUT_EN_W<EX_PHYINF_CONF_SPEC, 20> {
        TX_ERR_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Selection of MII/RMII phy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ex_phyinf_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ex_phyinf_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EX_PHYINF_CONF_SPEC;
impl crate::RegisterSpec for EX_PHYINF_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ex_phyinf_conf::R`](R) reader structure"]
impl crate::Readable for EX_PHYINF_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ex_phyinf_conf::W`](W) writer structure"]
impl crate::Writable for EX_PHYINF_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EX_PHYINF_CONF to value 0"]
impl crate::Resettable for EX_PHYINF_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
