#[doc = "Register `IN_PERI_SEL_CH%s` reader"]
pub type R = crate::R<IN_PERI_SEL_CH_SPEC>;
#[doc = "Register `IN_PERI_SEL_CH%s` writer"]
pub type W = crate::W<IN_PERI_SEL_CH_SPEC>;
#[doc = "Field `PERI_IN_SEL` reader - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
pub type PERI_IN_SEL_R = crate::FieldReader;
#[doc = "Field `PERI_IN_SEL` writer - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
pub type PERI_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    #[inline(always)]
    pub fn peri_in_sel(&self) -> PERI_IN_SEL_R {
        PERI_IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_PERI_SEL_CH")
            .field(
                "peri_in_sel",
                &format_args!("{}", self.peri_in_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_PERI_SEL_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    #[inline(always)]
    #[must_use]
    pub fn peri_in_sel(&mut self) -> PERI_IN_SEL_W<IN_PERI_SEL_CH_SPEC> {
        PERI_IN_SEL_W::new(self, 0)
    }
}
#[doc = "DMA_IN_PERI_SEL_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_PERI_SEL_CH_SPEC;
impl crate::RegisterSpec for IN_PERI_SEL_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_peri_sel_ch::R`](R) reader structure"]
impl crate::Readable for IN_PERI_SEL_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_peri_sel_ch::W`](W) writer structure"]
impl crate::Writable for IN_PERI_SEL_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_PERI_SEL_CH%s to value 0x3f"]
impl crate::Resettable for IN_PERI_SEL_CH_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}
