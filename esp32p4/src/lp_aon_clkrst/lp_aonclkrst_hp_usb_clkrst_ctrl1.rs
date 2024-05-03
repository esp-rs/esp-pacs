#[doc = "Register `LP_AONCLKRST_HP_USB_CLKRST_CTRL1` reader"]
pub type R = crate::R<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC>;
#[doc = "Register `LP_AONCLKRST_HP_USB_CLKRST_CTRL1` writer"]
pub type W = crate::W<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC>;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_OTG20_ADP` reader - usb otg20 adp reset en"]
pub type LP_AONCLKRST_RST_EN_USB_OTG20_ADP_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_OTG20_ADP` writer - usb otg20 adp reset en"]
pub type LP_AONCLKRST_RST_EN_USB_OTG20_ADP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_OTG20_PHY` reader - usb otg20 phy reset en"]
pub type LP_AONCLKRST_RST_EN_USB_OTG20_PHY_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_OTG20_PHY` writer - usb otg20 phy reset en"]
pub type LP_AONCLKRST_RST_EN_USB_OTG20_PHY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_OTG20` reader - usb otg20 reset en"]
pub type LP_AONCLKRST_RST_EN_USB_OTG20_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_OTG20` writer - usb otg20 reset en"]
pub type LP_AONCLKRST_RST_EN_USB_OTG20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_OTG11` reader - usb org11 reset en"]
pub type LP_AONCLKRST_RST_EN_USB_OTG11_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_OTG11` writer - usb org11 reset en"]
pub type LP_AONCLKRST_RST_EN_USB_OTG11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_DEVICE` reader - usb device reset en"]
pub type LP_AONCLKRST_RST_EN_USB_DEVICE_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_USB_DEVICE` writer - usb device reset en"]
pub type LP_AONCLKRST_RST_EN_USB_DEVICE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL` reader - usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk."]
pub type LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL` writer - usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk."]
pub type LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN` reader - usb otg20 hs phy refclk enable."]
pub type LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN` writer - usb otg20 hs phy refclk enable."]
pub type LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN` reader - usb otg20 ulpi clock enable."]
pub type LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN` writer - usb otg20 ulpi clock enable."]
pub type LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - usb otg20 adp reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_otg20_adp(&self) -> LP_AONCLKRST_RST_EN_USB_OTG20_ADP_R {
        LP_AONCLKRST_RST_EN_USB_OTG20_ADP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - usb otg20 phy reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_otg20_phy(&self) -> LP_AONCLKRST_RST_EN_USB_OTG20_PHY_R {
        LP_AONCLKRST_RST_EN_USB_OTG20_PHY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - usb otg20 reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_otg20(&self) -> LP_AONCLKRST_RST_EN_USB_OTG20_R {
        LP_AONCLKRST_RST_EN_USB_OTG20_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - usb org11 reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_otg11(&self) -> LP_AONCLKRST_RST_EN_USB_OTG11_R {
        LP_AONCLKRST_RST_EN_USB_OTG11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - usb device reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_device(&self) -> LP_AONCLKRST_RST_EN_USB_DEVICE_R {
        LP_AONCLKRST_RST_EN_USB_DEVICE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 28:29 - usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_phyref_clk_src_sel(
        &self,
    ) -> LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_R {
        LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - usb otg20 hs phy refclk enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_phyref_clk_en(&self) -> LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_R {
        LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - usb otg20 ulpi clock enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_ulpi_clk_en(&self) -> LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_R {
        LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_HP_USB_CLKRST_CTRL1")
            .field(
                "lp_aonclkrst_rst_en_usb_otg20_adp",
                &self.lp_aonclkrst_rst_en_usb_otg20_adp().bit(),
            )
            .field(
                "lp_aonclkrst_rst_en_usb_otg20_phy",
                &self.lp_aonclkrst_rst_en_usb_otg20_phy().bit(),
            )
            .field(
                "lp_aonclkrst_rst_en_usb_otg20",
                &self.lp_aonclkrst_rst_en_usb_otg20().bit(),
            )
            .field(
                "lp_aonclkrst_rst_en_usb_otg11",
                &self.lp_aonclkrst_rst_en_usb_otg11().bit(),
            )
            .field(
                "lp_aonclkrst_rst_en_usb_device",
                &self.lp_aonclkrst_rst_en_usb_device().bit(),
            )
            .field(
                "lp_aonclkrst_usb_otg20_phyref_clk_src_sel",
                &self.lp_aonclkrst_usb_otg20_phyref_clk_src_sel().bits(),
            )
            .field(
                "lp_aonclkrst_usb_otg20_phyref_clk_en",
                &self.lp_aonclkrst_usb_otg20_phyref_clk_en().bit(),
            )
            .field(
                "lp_aonclkrst_usb_otg20_ulpi_clk_en",
                &self.lp_aonclkrst_usb_otg20_ulpi_clk_en().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - usb otg20 adp reset en"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_rst_en_usb_otg20_adp(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_USB_OTG20_ADP_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC> {
        LP_AONCLKRST_RST_EN_USB_OTG20_ADP_W::new(self, 0)
    }
    #[doc = "Bit 1 - usb otg20 phy reset en"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_rst_en_usb_otg20_phy(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_USB_OTG20_PHY_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC> {
        LP_AONCLKRST_RST_EN_USB_OTG20_PHY_W::new(self, 1)
    }
    #[doc = "Bit 2 - usb otg20 reset en"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_rst_en_usb_otg20(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_USB_OTG20_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC> {
        LP_AONCLKRST_RST_EN_USB_OTG20_W::new(self, 2)
    }
    #[doc = "Bit 3 - usb org11 reset en"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_rst_en_usb_otg11(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_USB_OTG11_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC> {
        LP_AONCLKRST_RST_EN_USB_OTG11_W::new(self, 3)
    }
    #[doc = "Bit 4 - usb device reset en"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_rst_en_usb_device(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_USB_DEVICE_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC> {
        LP_AONCLKRST_RST_EN_USB_DEVICE_W::new(self, 4)
    }
    #[doc = "Bits 28:29 - usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_otg20_phyref_clk_src_sel(
        &mut self,
    ) -> LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC> {
        LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_W::new(self, 28)
    }
    #[doc = "Bit 30 - usb otg20 hs phy refclk enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_otg20_phyref_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC> {
        LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - usb otg20 ulpi clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_otg20_ulpi_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC> {
        LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_W::new(self, 31)
    }
}
#[doc = "HP USB Clock Reset Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hp_usb_clkrst_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_usb_clkrst_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hp_usb_clkrst_ctrl1::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hp_usb_clkrst_ctrl1::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HP_USB_CLKRST_CTRL1 to value 0xc000_0000"]
impl crate::Resettable for LP_AONCLKRST_HP_USB_CLKRST_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0xc000_0000;
}
