#[doc = "Register `USB_OTGHS_CTRL` reader"]
pub type R = crate::R<USB_OTGHS_CTRL_SPEC>;
#[doc = "Register `USB_OTGHS_CTRL` writer"]
pub type W = crate::W<USB_OTGHS_CTRL_SPEC>;
#[doc = "Field `REG_USB_OTGHS_PHY_PLL_FORCE_EN` reader - Set this bit as 1 to force use reg_usb_otghs_phy_pll_en"]
pub type REG_USB_OTGHS_PHY_PLL_FORCE_EN_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_PLL_FORCE_EN` writer - Set this bit as 1 to force use reg_usb_otghs_phy_pll_en"]
pub type REG_USB_OTGHS_PHY_PLL_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_PHY_PLL_EN` reader - pll_en for phy inf"]
pub type REG_USB_OTGHS_PHY_PLL_EN_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_PLL_EN` writer - pll_en for phy inf"]
pub type REG_USB_OTGHS_PHY_PLL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_PHY_SUSPENDM_FORCE_EN` reader - Set this bit as 1 to force use reg_usb_otghs_phy_suspendm"]
pub type REG_USB_OTGHS_PHY_SUSPENDM_FORCE_EN_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_SUSPENDM_FORCE_EN` writer - Set this bit as 1 to force use reg_usb_otghs_phy_suspendm"]
pub type REG_USB_OTGHS_PHY_SUSPENDM_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_PHY_SUSPENDM` reader - phy suspendm"]
pub type REG_USB_OTGHS_PHY_SUSPENDM_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_SUSPENDM` writer - phy suspendm"]
pub type REG_USB_OTGHS_PHY_SUSPENDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_PHY_RESET_FORCE_EN` reader - Set this bit as 1 to force use reg_usb_otghs_phy_reset"]
pub type REG_USB_OTGHS_PHY_RESET_FORCE_EN_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_RESET_FORCE_EN` writer - Set this bit as 1 to force use reg_usb_otghs_phy_reset"]
pub type REG_USB_OTGHS_PHY_RESET_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_PHY_RESET` reader - phy reset"]
pub type REG_USB_OTGHS_PHY_RESET_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_RESET` writer - phy reset"]
pub type REG_USB_OTGHS_PHY_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_PHY_TXBITSTUFF_EN` reader - phy txbitstuff_en"]
pub type REG_USB_OTGHS_PHY_TXBITSTUFF_EN_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_TXBITSTUFF_EN` writer - phy txbitstuff_en"]
pub type REG_USB_OTGHS_PHY_TXBITSTUFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_PHY_OTG_SUSPENDM` reader - phy otg_suspendm"]
pub type REG_USB_OTGHS_PHY_OTG_SUSPENDM_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_OTG_SUSPENDM` writer - phy otg_suspendm"]
pub type REG_USB_OTGHS_PHY_OTG_SUSPENDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_PHY_REFCLK_MODE` reader - phy refclk_mode"]
pub type REG_USB_OTGHS_PHY_REFCLK_MODE_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_REFCLK_MODE` writer - phy refclk_mode"]
pub type REG_USB_OTGHS_PHY_REFCLK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_CORE_SS_SCALEDOWN_MODE` reader - usb otghs core ss_scaledown_mode"]
pub type REG_USB_OTGHS_CORE_SS_SCALEDOWN_MODE_R = crate::FieldReader;
#[doc = "Field `REG_USB_OTGHS_CORE_SS_SCALEDOWN_MODE` writer - usb otghs core ss_scaledown_mode"]
pub type REG_USB_OTGHS_CORE_SS_SCALEDOWN_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_USB_OTGHS_PHY_SELF_TEST` reader - phy self_test"]
pub type REG_USB_OTGHS_PHY_SELF_TEST_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_SELF_TEST` writer - phy self_test"]
pub type REG_USB_OTGHS_PHY_SELF_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_USB_OTGHS_PHY_TEST_DONE` reader - phy test_bist"]
pub type REG_USB_OTGHS_PHY_TEST_DONE_R = crate::BitReader;
#[doc = "Field `REG_USB_OTGHS_PHY_DTO` reader - phy dto"]
pub type REG_USB_OTGHS_PHY_DTO_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit as 1 to force use reg_usb_otghs_phy_pll_en"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_pll_force_en(&self) -> REG_USB_OTGHS_PHY_PLL_FORCE_EN_R {
        REG_USB_OTGHS_PHY_PLL_FORCE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pll_en for phy inf"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_pll_en(&self) -> REG_USB_OTGHS_PHY_PLL_EN_R {
        REG_USB_OTGHS_PHY_PLL_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit as 1 to force use reg_usb_otghs_phy_suspendm"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_suspendm_force_en(&self) -> REG_USB_OTGHS_PHY_SUSPENDM_FORCE_EN_R {
        REG_USB_OTGHS_PHY_SUSPENDM_FORCE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - phy suspendm"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_suspendm(&self) -> REG_USB_OTGHS_PHY_SUSPENDM_R {
        REG_USB_OTGHS_PHY_SUSPENDM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit as 1 to force use reg_usb_otghs_phy_reset"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_reset_force_en(&self) -> REG_USB_OTGHS_PHY_RESET_FORCE_EN_R {
        REG_USB_OTGHS_PHY_RESET_FORCE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - phy reset"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_reset(&self) -> REG_USB_OTGHS_PHY_RESET_R {
        REG_USB_OTGHS_PHY_RESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - phy txbitstuff_en"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_txbitstuff_en(&self) -> REG_USB_OTGHS_PHY_TXBITSTUFF_EN_R {
        REG_USB_OTGHS_PHY_TXBITSTUFF_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - phy otg_suspendm"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_otg_suspendm(&self) -> REG_USB_OTGHS_PHY_OTG_SUSPENDM_R {
        REG_USB_OTGHS_PHY_OTG_SUSPENDM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - phy refclk_mode"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_refclk_mode(&self) -> REG_USB_OTGHS_PHY_REFCLK_MODE_R {
        REG_USB_OTGHS_PHY_REFCLK_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - usb otghs core ss_scaledown_mode"]
    #[inline(always)]
    pub fn reg_usb_otghs_core_ss_scaledown_mode(&self) -> REG_USB_OTGHS_CORE_SS_SCALEDOWN_MODE_R {
        REG_USB_OTGHS_CORE_SS_SCALEDOWN_MODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - phy self_test"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_self_test(&self) -> REG_USB_OTGHS_PHY_SELF_TEST_R {
        REG_USB_OTGHS_PHY_SELF_TEST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - phy test_bist"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_test_done(&self) -> REG_USB_OTGHS_PHY_TEST_DONE_R {
        REG_USB_OTGHS_PHY_TEST_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - phy dto"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_dto(&self) -> REG_USB_OTGHS_PHY_DTO_R {
        REG_USB_OTGHS_PHY_DTO_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_OTGHS_CTRL")
            .field(
                "reg_usb_otghs_phy_pll_force_en",
                &self.reg_usb_otghs_phy_pll_force_en(),
            )
            .field("reg_usb_otghs_phy_pll_en", &self.reg_usb_otghs_phy_pll_en())
            .field(
                "reg_usb_otghs_phy_suspendm_force_en",
                &self.reg_usb_otghs_phy_suspendm_force_en(),
            )
            .field(
                "reg_usb_otghs_phy_suspendm",
                &self.reg_usb_otghs_phy_suspendm(),
            )
            .field(
                "reg_usb_otghs_phy_reset_force_en",
                &self.reg_usb_otghs_phy_reset_force_en(),
            )
            .field("reg_usb_otghs_phy_reset", &self.reg_usb_otghs_phy_reset())
            .field(
                "reg_usb_otghs_phy_txbitstuff_en",
                &self.reg_usb_otghs_phy_txbitstuff_en(),
            )
            .field(
                "reg_usb_otghs_phy_otg_suspendm",
                &self.reg_usb_otghs_phy_otg_suspendm(),
            )
            .field(
                "reg_usb_otghs_phy_refclk_mode",
                &self.reg_usb_otghs_phy_refclk_mode(),
            )
            .field(
                "reg_usb_otghs_core_ss_scaledown_mode",
                &self.reg_usb_otghs_core_ss_scaledown_mode(),
            )
            .field(
                "reg_usb_otghs_phy_self_test",
                &self.reg_usb_otghs_phy_self_test(),
            )
            .field(
                "reg_usb_otghs_phy_test_done",
                &self.reg_usb_otghs_phy_test_done(),
            )
            .field("reg_usb_otghs_phy_dto", &self.reg_usb_otghs_phy_dto())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit as 1 to force use reg_usb_otghs_phy_pll_en"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_pll_force_en(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_PLL_FORCE_EN_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_PLL_FORCE_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - pll_en for phy inf"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_pll_en(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_PLL_EN_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_PLL_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit as 1 to force use reg_usb_otghs_phy_suspendm"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_suspendm_force_en(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_SUSPENDM_FORCE_EN_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_SUSPENDM_FORCE_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - phy suspendm"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_suspendm(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_SUSPENDM_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_SUSPENDM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit as 1 to force use reg_usb_otghs_phy_reset"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_reset_force_en(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_RESET_FORCE_EN_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_RESET_FORCE_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - phy reset"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_reset(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_RESET_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_RESET_W::new(self, 5)
    }
    #[doc = "Bit 6 - phy txbitstuff_en"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_txbitstuff_en(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_TXBITSTUFF_EN_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_TXBITSTUFF_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - phy otg_suspendm"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_otg_suspendm(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_OTG_SUSPENDM_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_OTG_SUSPENDM_W::new(self, 7)
    }
    #[doc = "Bit 8 - phy refclk_mode"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_refclk_mode(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_REFCLK_MODE_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_REFCLK_MODE_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - usb otghs core ss_scaledown_mode"]
    #[inline(always)]
    pub fn reg_usb_otghs_core_ss_scaledown_mode(
        &mut self,
    ) -> REG_USB_OTGHS_CORE_SS_SCALEDOWN_MODE_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_CORE_SS_SCALEDOWN_MODE_W::new(self, 9)
    }
    #[doc = "Bit 11 - phy self_test"]
    #[inline(always)]
    pub fn reg_usb_otghs_phy_self_test(
        &mut self,
    ) -> REG_USB_OTGHS_PHY_SELF_TEST_W<'_, USB_OTGHS_CTRL_SPEC> {
        REG_USB_OTGHS_PHY_SELF_TEST_W::new(self, 11)
    }
}
#[doc = "usb_otghs Control configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otghs_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_otghs_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_OTGHS_CTRL_SPEC;
impl crate::RegisterSpec for USB_OTGHS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_otghs_ctrl::R`](R) reader structure"]
impl crate::Readable for USB_OTGHS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_otghs_ctrl::W`](W) writer structure"]
impl crate::Writable for USB_OTGHS_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_OTGHS_CTRL to value 0x0104"]
impl crate::Resettable for USB_OTGHS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0104;
}
