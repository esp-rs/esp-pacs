#[doc = "Register `OUT_PERI_SEL_CH%s` reader"]
pub struct R(crate::R<OUT_PERI_SEL_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PERI_SEL_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PERI_SEL_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PERI_SEL_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PERI_SEL_CH%s` writer"]
pub struct W(crate::W<OUT_PERI_SEL_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PERI_SEL_CH_SPEC>;
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
impl From<crate::W<OUT_PERI_SEL_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PERI_SEL_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_OUT_SEL` reader - This register is used to select peripheral for Tx channel 0. 0:SPI2. 1: SPI3. 2: UHCI0. 3: I2S0. 4: I2S1. 5: LCD_CAM. 6: AES. 7: SHA. 8: ADC_DAC. 9: RMT."]
pub type PERI_OUT_SEL_R = crate::FieldReader;
#[doc = "Field `PERI_OUT_SEL` writer - This register is used to select peripheral for Tx channel 0. 0:SPI2. 1: SPI3. 2: UHCI0. 3: I2S0. 4: I2S1. 5: LCD_CAM. 6: AES. 7: SHA. 8: ADC_DAC. 9: RMT."]
pub type PERI_OUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, OUT_PERI_SEL_CH_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Tx channel 0. 0:SPI2. 1: SPI3. 2: UHCI0. 3: I2S0. 4: I2S1. 5: LCD_CAM. 6: AES. 7: SHA. 8: ADC_DAC. 9: RMT."]
    #[inline(always)]
    pub fn peri_out_sel(&self) -> PERI_OUT_SEL_R {
        PERI_OUT_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PERI_SEL_CH")
            .field(
                "peri_out_sel",
                &format_args!("{}", self.peri_out_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_PERI_SEL_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Tx channel 0. 0:SPI2. 1: SPI3. 2: UHCI0. 3: I2S0. 4: I2S1. 5: LCD_CAM. 6: AES. 7: SHA. 8: ADC_DAC. 9: RMT."]
    #[inline(always)]
    #[must_use]
    pub fn peri_out_sel(&mut self) -> PERI_OUT_SEL_W<0> {
        PERI_OUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral selection of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_peri_sel_ch](index.html) module"]
pub struct OUT_PERI_SEL_CH_SPEC;
impl crate::RegisterSpec for OUT_PERI_SEL_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_peri_sel_ch::R](R) reader structure"]
impl crate::Readable for OUT_PERI_SEL_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_peri_sel_ch::W](W) writer structure"]
impl crate::Writable for OUT_PERI_SEL_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_PERI_SEL_CH%s to value 0x3f"]
impl crate::Resettable for OUT_PERI_SEL_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
