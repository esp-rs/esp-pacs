#[doc = "Register `IN_PERI_SEL_CH0` reader"]
pub struct R(crate::R<IN_PERI_SEL_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_PERI_SEL_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_PERI_SEL_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_PERI_SEL_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_PERI_SEL_CH0` writer"]
pub struct W(crate::W<IN_PERI_SEL_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_PERI_SEL_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IN_PERI_SEL_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_PERI_SEL_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_IN_SEL` reader - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
pub type PERI_IN_SEL_R = crate::FieldReader;
#[doc = "Field `PERI_IN_SEL` writer - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
pub type PERI_IN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, IN_PERI_SEL_CH0_SPEC, 6, O>;
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
        f.debug_struct("IN_PERI_SEL_CH0")
            .field(
                "peri_in_sel",
                &format_args!("{}", self.peri_in_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_PERI_SEL_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    #[inline(always)]
    #[must_use]
    pub fn peri_in_sel(&mut self) -> PERI_IN_SEL_W<0> {
        PERI_IN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_PERI_SEL_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_peri_sel_ch0](index.html) module"]
pub struct IN_PERI_SEL_CH0_SPEC;
impl crate::RegisterSpec for IN_PERI_SEL_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_peri_sel_ch0::R](R) reader structure"]
impl crate::Readable for IN_PERI_SEL_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_peri_sel_ch0::W](W) writer structure"]
impl crate::Writable for IN_PERI_SEL_CH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_PERI_SEL_CH0 to value 0x3f"]
impl crate::Resettable for IN_PERI_SEL_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
