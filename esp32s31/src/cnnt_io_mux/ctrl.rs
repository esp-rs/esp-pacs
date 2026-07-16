#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SDIO_PAD_PIN_CTRL_DED_SEL` reader - Configures whether or not to use dedicated ctrl signals for SDIO PAD.\\\\ 0: Not use dedicated ctrl signals\\\\ 1: Use dedicated ctrl signals\\\\"]
pub type SDIO_PAD_PIN_CTRL_DED_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_PIN_CTRL_DED_SEL` writer - Configures whether or not to use dedicated ctrl signals for SDIO PAD.\\\\ 0: Not use dedicated ctrl signals\\\\ 1: Use dedicated ctrl signals\\\\"]
pub type SDIO_PAD_PIN_CTRL_DED_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMAC_PAD_PIN_CTRL_DED_SEL` reader - Configures whether or not to use dedicated ctrl signals for GMAC PAD.\\\\ 0: Not use dedicated ctrl signals\\\\ 1: Use dedicated ctrl signals\\\\"]
pub type GMAC_PAD_PIN_CTRL_DED_SEL_R = crate::BitReader;
#[doc = "Field `GMAC_PAD_PIN_CTRL_DED_SEL` writer - Configures whether or not to use dedicated ctrl signals for GMAC PAD.\\\\ 0: Not use dedicated ctrl signals\\\\ 1: Use dedicated ctrl signals\\\\"]
pub type GMAC_PAD_PIN_CTRL_DED_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to use dedicated ctrl signals for SDIO PAD.\\\\ 0: Not use dedicated ctrl signals\\\\ 1: Use dedicated ctrl signals\\\\"]
    #[inline(always)]
    pub fn sdio_pad_pin_ctrl_ded_sel(&self) -> SDIO_PAD_PIN_CTRL_DED_SEL_R {
        SDIO_PAD_PIN_CTRL_DED_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to use dedicated ctrl signals for GMAC PAD.\\\\ 0: Not use dedicated ctrl signals\\\\ 1: Use dedicated ctrl signals\\\\"]
    #[inline(always)]
    pub fn gmac_pad_pin_ctrl_ded_sel(&self) -> GMAC_PAD_PIN_CTRL_DED_SEL_R {
        GMAC_PAD_PIN_CTRL_DED_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field(
                "sdio_pad_pin_ctrl_ded_sel",
                &self.sdio_pad_pin_ctrl_ded_sel(),
            )
            .field(
                "gmac_pad_pin_ctrl_ded_sel",
                &self.gmac_pad_pin_ctrl_ded_sel(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to use dedicated ctrl signals for SDIO PAD.\\\\ 0: Not use dedicated ctrl signals\\\\ 1: Use dedicated ctrl signals\\\\"]
    #[inline(always)]
    pub fn sdio_pad_pin_ctrl_ded_sel(&mut self) -> SDIO_PAD_PIN_CTRL_DED_SEL_W<'_, CTRL_SPEC> {
        SDIO_PAD_PIN_CTRL_DED_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to use dedicated ctrl signals for GMAC PAD.\\\\ 0: Not use dedicated ctrl signals\\\\ 1: Use dedicated ctrl signals\\\\"]
    #[inline(always)]
    pub fn gmac_pad_pin_ctrl_ded_sel(&mut self) -> GMAC_PAD_PIN_CTRL_DED_SEL_W<'_, CTRL_SPEC> {
        GMAC_PAD_PIN_CTRL_DED_SEL_W::new(self, 1)
    }
}
#[doc = "SDIO IO MUX configuration register for ctrl sel\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {}
