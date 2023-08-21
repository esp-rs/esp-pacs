#[doc = "Register `IN_PERI_SEL_CH1` reader"]
pub type R = crate::R<IN_PERI_SEL_CH1_SPEC>;
#[doc = "Register `IN_PERI_SEL_CH1` writer"]
pub type W = crate::W<IN_PERI_SEL_CH1_SPEC>;
#[doc = "Field `PERI_IN_SEL` reader - This register is used to select peripheral for Rx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
pub type PERI_IN_SEL_R = crate::FieldReader;
#[doc = "Field `PERI_IN_SEL` writer - This register is used to select peripheral for Rx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
pub type PERI_IN_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    #[inline(always)]
    pub fn peri_in_sel(&self) -> PERI_IN_SEL_R {
        PERI_IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_PERI_SEL_CH1")
            .field(
                "peri_in_sel",
                &format_args!("{}", self.peri_in_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_PERI_SEL_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    #[inline(always)]
    #[must_use]
    pub fn peri_in_sel(&mut self) -> PERI_IN_SEL_W<IN_PERI_SEL_CH1_SPEC, 0> {
        PERI_IN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA_IN_PERI_SEL_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_PERI_SEL_CH1_SPEC;
impl crate::RegisterSpec for IN_PERI_SEL_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_peri_sel_ch1::R`](R) reader structure"]
impl crate::Readable for IN_PERI_SEL_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_peri_sel_ch1::W`](W) writer structure"]
impl crate::Writable for IN_PERI_SEL_CH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_PERI_SEL_CH1 to value 0x3f"]
impl crate::Resettable for IN_PERI_SEL_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
