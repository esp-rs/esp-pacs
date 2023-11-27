#[doc = "Register `GMAC_CTRL0` reader"]
pub type R = crate::R<GMAC_CTRL0_SPEC>;
#[doc = "Register `GMAC_CTRL0` writer"]
pub type W = crate::W<GMAC_CTRL0_SPEC>;
#[doc = "Field `PTP_PPS` reader - N/A"]
pub type PTP_PPS_R = crate::BitReader;
#[doc = "Field `SBD_FLOWCTRL` reader - N/A"]
pub type SBD_FLOWCTRL_R = crate::BitReader;
#[doc = "Field `SBD_FLOWCTRL` writer - N/A"]
pub type SBD_FLOWCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_INTF_SEL` reader - N/A"]
pub type PHY_INTF_SEL_R = crate::FieldReader;
#[doc = "Field `PHY_INTF_SEL` writer - N/A"]
pub type PHY_INTF_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GMAC_MEM_CLK_FORCE_ON` reader - N/A"]
pub type GMAC_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `GMAC_MEM_CLK_FORCE_ON` writer - N/A"]
pub type GMAC_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMAC_RST_CLK_TX_N` reader - N/A"]
pub type GMAC_RST_CLK_TX_N_R = crate::BitReader;
#[doc = "Field `GMAC_RST_CLK_RX_N` reader - N/A"]
pub type GMAC_RST_CLK_RX_N_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn ptp_pps(&self) -> PTP_PPS_R {
        PTP_PPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn sbd_flowctrl(&self) -> SBD_FLOWCTRL_R {
        SBD_FLOWCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - N/A"]
    #[inline(always)]
    pub fn phy_intf_sel(&self) -> PHY_INTF_SEL_R {
        PHY_INTF_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn gmac_mem_clk_force_on(&self) -> GMAC_MEM_CLK_FORCE_ON_R {
        GMAC_MEM_CLK_FORCE_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn gmac_rst_clk_tx_n(&self) -> GMAC_RST_CLK_TX_N_R {
        GMAC_RST_CLK_TX_N_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn gmac_rst_clk_rx_n(&self) -> GMAC_RST_CLK_RX_N_R {
        GMAC_RST_CLK_RX_N_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GMAC_CTRL0")
            .field("ptp_pps", &format_args!("{}", self.ptp_pps().bit()))
            .field(
                "sbd_flowctrl",
                &format_args!("{}", self.sbd_flowctrl().bit()),
            )
            .field(
                "phy_intf_sel",
                &format_args!("{}", self.phy_intf_sel().bits()),
            )
            .field(
                "gmac_mem_clk_force_on",
                &format_args!("{}", self.gmac_mem_clk_force_on().bit()),
            )
            .field(
                "gmac_rst_clk_tx_n",
                &format_args!("{}", self.gmac_rst_clk_tx_n().bit()),
            )
            .field(
                "gmac_rst_clk_rx_n",
                &format_args!("{}", self.gmac_rst_clk_rx_n().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GMAC_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sbd_flowctrl(&mut self) -> SBD_FLOWCTRL_W<GMAC_CTRL0_SPEC> {
        SBD_FLOWCTRL_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn phy_intf_sel(&mut self) -> PHY_INTF_SEL_W<GMAC_CTRL0_SPEC> {
        PHY_INTF_SEL_W::new(self, 2)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_mem_clk_force_on(&mut self) -> GMAC_MEM_CLK_FORCE_ON_W<GMAC_CTRL0_SPEC> {
        GMAC_MEM_CLK_FORCE_ON_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC_CTRL0_SPEC;
impl crate::RegisterSpec for GMAC_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_ctrl0::R`](R) reader structure"]
impl crate::Readable for GMAC_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmac_ctrl0::W`](W) writer structure"]
impl crate::Writable for GMAC_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GMAC_CTRL0 to value 0"]
impl crate::Resettable for GMAC_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
