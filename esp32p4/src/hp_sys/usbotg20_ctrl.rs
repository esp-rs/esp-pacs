#[doc = "Register `USBOTG20_CTRL` reader"]
pub type R = crate::R<USBOTG20_CTRL_SPEC>;
#[doc = "Register `USBOTG20_CTRL` writer"]
pub type W = crate::W<USBOTG20_CTRL_SPEC>;
#[doc = "Field `OTG_PHY_TEST_DONE` reader - N/A"]
pub type OTG_PHY_TEST_DONE_R = crate::BitReader;
#[doc = "Field `USB_MEM_AUX_CTRL` reader - N/A"]
pub type USB_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `USB_MEM_AUX_CTRL` writer - N/A"]
pub type USB_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PHY_SUSPENDM` reader - N/A"]
pub type PHY_SUSPENDM_R = crate::BitReader;
#[doc = "Field `PHY_SUSPENDM` writer - N/A"]
pub type PHY_SUSPENDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_SUSPEND_FORCE_EN` reader - N/A"]
pub type PHY_SUSPEND_FORCE_EN_R = crate::BitReader;
#[doc = "Field `PHY_SUSPEND_FORCE_EN` writer - N/A"]
pub type PHY_SUSPEND_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RSTN` reader - N/A"]
pub type PHY_RSTN_R = crate::BitReader;
#[doc = "Field `PHY_RSTN` writer - N/A"]
pub type PHY_RSTN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RESET_FORCE_EN` reader - N/A"]
pub type PHY_RESET_FORCE_EN_R = crate::BitReader;
#[doc = "Field `PHY_RESET_FORCE_EN` writer - N/A"]
pub type PHY_RESET_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PLL_FORCE_EN` reader - N/A"]
pub type PHY_PLL_FORCE_EN_R = crate::BitReader;
#[doc = "Field `PHY_PLL_FORCE_EN` writer - N/A"]
pub type PHY_PLL_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PLL_EN` reader - N/A"]
pub type PHY_PLL_EN_R = crate::BitReader;
#[doc = "Field `PHY_PLL_EN` writer - N/A"]
pub type PHY_PLL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_SUSPENDM` reader - N/A"]
pub type OTG_SUSPENDM_R = crate::BitReader;
#[doc = "Field `OTG_SUSPENDM` writer - N/A"]
pub type OTG_SUSPENDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_PHY_TXBITSTUFF_EN` reader - N/A"]
pub type OTG_PHY_TXBITSTUFF_EN_R = crate::BitReader;
#[doc = "Field `OTG_PHY_TXBITSTUFF_EN` writer - N/A"]
pub type OTG_PHY_TXBITSTUFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_PHY_REFCLK_MODE` reader - N/A"]
pub type OTG_PHY_REFCLK_MODE_R = crate::BitReader;
#[doc = "Field `OTG_PHY_REFCLK_MODE` writer - N/A"]
pub type OTG_PHY_REFCLK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_PHY_BISTEN` reader - N/A"]
pub type OTG_PHY_BISTEN_R = crate::BitReader;
#[doc = "Field `OTG_PHY_BISTEN` writer - N/A"]
pub type OTG_PHY_BISTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn otg_phy_test_done(&self) -> OTG_PHY_TEST_DONE_R {
        OTG_PHY_TEST_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - N/A"]
    #[inline(always)]
    pub fn usb_mem_aux_ctrl(&self) -> USB_MEM_AUX_CTRL_R {
        USB_MEM_AUX_CTRL_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn phy_suspendm(&self) -> PHY_SUSPENDM_R {
        PHY_SUSPENDM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn phy_suspend_force_en(&self) -> PHY_SUSPEND_FORCE_EN_R {
        PHY_SUSPEND_FORCE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn phy_rstn(&self) -> PHY_RSTN_R {
        PHY_RSTN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn phy_reset_force_en(&self) -> PHY_RESET_FORCE_EN_R {
        PHY_RESET_FORCE_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn phy_pll_force_en(&self) -> PHY_PLL_FORCE_EN_R {
        PHY_PLL_FORCE_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn phy_pll_en(&self) -> PHY_PLL_EN_R {
        PHY_PLL_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn otg_suspendm(&self) -> OTG_SUSPENDM_R {
        OTG_SUSPENDM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    pub fn otg_phy_txbitstuff_en(&self) -> OTG_PHY_TXBITSTUFF_EN_R {
        OTG_PHY_TXBITSTUFF_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn otg_phy_refclk_mode(&self) -> OTG_PHY_REFCLK_MODE_R {
        OTG_PHY_REFCLK_MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn otg_phy_bisten(&self) -> OTG_PHY_BISTEN_R {
        OTG_PHY_BISTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBOTG20_CTRL")
            .field("otg_phy_test_done", &self.otg_phy_test_done())
            .field("usb_mem_aux_ctrl", &self.usb_mem_aux_ctrl())
            .field("phy_suspendm", &self.phy_suspendm())
            .field("phy_suspend_force_en", &self.phy_suspend_force_en())
            .field("phy_rstn", &self.phy_rstn())
            .field("phy_reset_force_en", &self.phy_reset_force_en())
            .field("phy_pll_force_en", &self.phy_pll_force_en())
            .field("phy_pll_en", &self.phy_pll_en())
            .field("otg_suspendm", &self.otg_suspendm())
            .field("otg_phy_txbitstuff_en", &self.otg_phy_txbitstuff_en())
            .field("otg_phy_refclk_mode", &self.otg_phy_refclk_mode())
            .field("otg_phy_bisten", &self.otg_phy_bisten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:14 - N/A"]
    #[inline(always)]
    pub fn usb_mem_aux_ctrl(&mut self) -> USB_MEM_AUX_CTRL_W<USBOTG20_CTRL_SPEC> {
        USB_MEM_AUX_CTRL_W::new(self, 1)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn phy_suspendm(&mut self) -> PHY_SUSPENDM_W<USBOTG20_CTRL_SPEC> {
        PHY_SUSPENDM_W::new(self, 15)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn phy_suspend_force_en(&mut self) -> PHY_SUSPEND_FORCE_EN_W<USBOTG20_CTRL_SPEC> {
        PHY_SUSPEND_FORCE_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn phy_rstn(&mut self) -> PHY_RSTN_W<USBOTG20_CTRL_SPEC> {
        PHY_RSTN_W::new(self, 17)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn phy_reset_force_en(&mut self) -> PHY_RESET_FORCE_EN_W<USBOTG20_CTRL_SPEC> {
        PHY_RESET_FORCE_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn phy_pll_force_en(&mut self) -> PHY_PLL_FORCE_EN_W<USBOTG20_CTRL_SPEC> {
        PHY_PLL_FORCE_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn phy_pll_en(&mut self) -> PHY_PLL_EN_W<USBOTG20_CTRL_SPEC> {
        PHY_PLL_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn otg_suspendm(&mut self) -> OTG_SUSPENDM_W<USBOTG20_CTRL_SPEC> {
        OTG_SUSPENDM_W::new(self, 21)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    pub fn otg_phy_txbitstuff_en(&mut self) -> OTG_PHY_TXBITSTUFF_EN_W<USBOTG20_CTRL_SPEC> {
        OTG_PHY_TXBITSTUFF_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn otg_phy_refclk_mode(&mut self) -> OTG_PHY_REFCLK_MODE_W<USBOTG20_CTRL_SPEC> {
        OTG_PHY_REFCLK_MODE_W::new(self, 23)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn otg_phy_bisten(&mut self) -> OTG_PHY_BISTEN_W<USBOTG20_CTRL_SPEC> {
        OTG_PHY_BISTEN_W::new(self, 24)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`usbotg20_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbotg20_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBOTG20_CTRL_SPEC;
impl crate::RegisterSpec for USBOTG20_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbotg20_ctrl::R`](R) reader structure"]
impl crate::Readable for USBOTG20_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbotg20_ctrl::W`](W) writer structure"]
impl crate::Writable for USBOTG20_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBOTG20_CTRL to value 0x0082_2640"]
impl crate::Resettable for USBOTG20_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0082_2640;
}
