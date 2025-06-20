#[doc = "Register `OUT_PERI_SEL_CH%s` reader"]
pub type R = crate::R<OUT_PERI_SEL_CH_SPEC>;
#[doc = "Register `OUT_PERI_SEL_CH%s` writer"]
pub type W = crate::W<OUT_PERI_SEL_CH_SPEC>;
#[doc = "Field `PERI_OUT_SEL_CH` reader - Configures the peripheral connected to TX channel %s.\\\\ 0: Dummy\\\\ 1: SPI2\\\\ 2: UHCI0\\\\ 3: I2S0\\\\ 4: Dummy\\\\ 5: Dummy\\\\ 6: AES\\\\ 7: SHA\\\\ 8: ADC_DAC\\\\ 9: PARL_IO\\\\ 10: Dummy\\\\ 11~15: Dummy\\\\"]
pub type PERI_OUT_SEL_CH_R = crate::FieldReader;
#[doc = "Field `PERI_OUT_SEL_CH` writer - Configures the peripheral connected to TX channel %s.\\\\ 0: Dummy\\\\ 1: SPI2\\\\ 2: UHCI0\\\\ 3: I2S0\\\\ 4: Dummy\\\\ 5: Dummy\\\\ 6: AES\\\\ 7: SHA\\\\ 8: ADC_DAC\\\\ 9: PARL_IO\\\\ 10: Dummy\\\\ 11~15: Dummy\\\\"]
pub type PERI_OUT_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures the peripheral connected to TX channel %s.\\\\ 0: Dummy\\\\ 1: SPI2\\\\ 2: UHCI0\\\\ 3: I2S0\\\\ 4: Dummy\\\\ 5: Dummy\\\\ 6: AES\\\\ 7: SHA\\\\ 8: ADC_DAC\\\\ 9: PARL_IO\\\\ 10: Dummy\\\\ 11~15: Dummy\\\\"]
    #[inline(always)]
    pub fn peri_out_sel_ch(&self) -> PERI_OUT_SEL_CH_R {
        PERI_OUT_SEL_CH_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PERI_SEL_CH")
            .field("peri_out_sel_ch", &self.peri_out_sel_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the peripheral connected to TX channel %s.\\\\ 0: Dummy\\\\ 1: SPI2\\\\ 2: UHCI0\\\\ 3: I2S0\\\\ 4: Dummy\\\\ 5: Dummy\\\\ 6: AES\\\\ 7: SHA\\\\ 8: ADC_DAC\\\\ 9: PARL_IO\\\\ 10: Dummy\\\\ 11~15: Dummy\\\\"]
    #[inline(always)]
    pub fn peri_out_sel_ch(&mut self) -> PERI_OUT_SEL_CH_W<OUT_PERI_SEL_CH_SPEC> {
        PERI_OUT_SEL_CH_W::new(self, 0)
    }
}
#[doc = "Peripheral selection register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_sel_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_sel_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PERI_SEL_CH_SPEC;
impl crate::RegisterSpec for OUT_PERI_SEL_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_peri_sel_ch::R`](R) reader structure"]
impl crate::Readable for OUT_PERI_SEL_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_peri_sel_ch::W`](W) writer structure"]
impl crate::Writable for OUT_PERI_SEL_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_PERI_SEL_CH%s to value 0x3f"]
impl crate::Resettable for OUT_PERI_SEL_CH_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}
