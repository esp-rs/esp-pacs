#[doc = "Register `FILTER_CTRL` reader"]
pub struct R(crate::R<FILTER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER_CTRL` writer"]
pub struct W(crate::W<FILTER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_CTRL_SPEC>;
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
impl From<crate::W<FILTER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC2_FILTER_RESET` reader - Reset ADC2 filter."]
pub type ADC2_FILTER_RESET_R = crate::BitReader;
#[doc = "Field `ADC2_FILTER_RESET` writer - Reset ADC2 filter."]
pub type ADC2_FILTER_RESET_W<'a, const O: u8> = crate::BitWriter<'a, FILTER_CTRL_SPEC, O>;
#[doc = "Field `ADC1_FILTER_RESET` reader - Reset ADC1 filter."]
pub type ADC1_FILTER_RESET_R = crate::BitReader;
#[doc = "Field `ADC1_FILTER_RESET` writer - Reset ADC1 filter."]
pub type ADC1_FILTER_RESET_W<'a, const O: u8> = crate::BitWriter<'a, FILTER_CTRL_SPEC, O>;
#[doc = "Field `ADC2_FILTER_FACTOR` reader - Set filter factor for DIG ADC2 CRTL."]
pub type ADC2_FILTER_FACTOR_R = crate::FieldReader;
#[doc = "Field `ADC2_FILTER_FACTOR` writer - Set filter factor for DIG ADC2 CRTL."]
pub type ADC2_FILTER_FACTOR_W<'a, const O: u8> = crate::FieldWriter<'a, FILTER_CTRL_SPEC, 7, O>;
#[doc = "Field `ADC1_FILTER_FACTOR` reader - Set filter factor for DIG ADC1 CRTL."]
pub type ADC1_FILTER_FACTOR_R = crate::FieldReader;
#[doc = "Field `ADC1_FILTER_FACTOR` writer - Set filter factor for DIG ADC1 CRTL."]
pub type ADC1_FILTER_FACTOR_W<'a, const O: u8> = crate::FieldWriter<'a, FILTER_CTRL_SPEC, 7, O>;
#[doc = "Field `ADC2_FILTER_EN` reader - Enable DIG ADC2 CRTL filter."]
pub type ADC2_FILTER_EN_R = crate::BitReader;
#[doc = "Field `ADC2_FILTER_EN` writer - Enable DIG ADC2 CRTL filter."]
pub type ADC2_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, FILTER_CTRL_SPEC, O>;
#[doc = "Field `ADC1_FILTER_EN` reader - Enable DIG ADC1 CRTL filter."]
pub type ADC1_FILTER_EN_R = crate::BitReader;
#[doc = "Field `ADC1_FILTER_EN` writer - Enable DIG ADC1 CRTL filter."]
pub type ADC1_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, FILTER_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Reset ADC2 filter."]
    #[inline(always)]
    pub fn adc2_filter_reset(&self) -> ADC2_FILTER_RESET_R {
        ADC2_FILTER_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset ADC1 filter."]
    #[inline(always)]
    pub fn adc1_filter_reset(&self) -> ADC1_FILTER_RESET_R {
        ADC1_FILTER_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Set filter factor for DIG ADC2 CRTL."]
    #[inline(always)]
    pub fn adc2_filter_factor(&self) -> ADC2_FILTER_FACTOR_R {
        ADC2_FILTER_FACTOR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:29 - Set filter factor for DIG ADC1 CRTL."]
    #[inline(always)]
    pub fn adc1_filter_factor(&self) -> ADC1_FILTER_FACTOR_R {
        ADC1_FILTER_FACTOR_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - Enable DIG ADC2 CRTL filter."]
    #[inline(always)]
    pub fn adc2_filter_en(&self) -> ADC2_FILTER_EN_R {
        ADC2_FILTER_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable DIG ADC1 CRTL filter."]
    #[inline(always)]
    pub fn adc1_filter_en(&self) -> ADC1_FILTER_EN_R {
        ADC1_FILTER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CTRL")
            .field(
                "adc2_filter_reset",
                &format_args!("{}", self.adc2_filter_reset().bit()),
            )
            .field(
                "adc1_filter_reset",
                &format_args!("{}", self.adc1_filter_reset().bit()),
            )
            .field(
                "adc2_filter_factor",
                &format_args!("{}", self.adc2_filter_factor().bits()),
            )
            .field(
                "adc1_filter_factor",
                &format_args!("{}", self.adc1_filter_factor().bits()),
            )
            .field(
                "adc2_filter_en",
                &format_args!("{}", self.adc2_filter_en().bit()),
            )
            .field(
                "adc1_filter_en",
                &format_args!("{}", self.adc1_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FILTER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Reset ADC2 filter."]
    #[inline(always)]
    #[must_use]
    pub fn adc2_filter_reset(&mut self) -> ADC2_FILTER_RESET_W<0> {
        ADC2_FILTER_RESET_W::new(self)
    }
    #[doc = "Bit 1 - Reset ADC1 filter."]
    #[inline(always)]
    #[must_use]
    pub fn adc1_filter_reset(&mut self) -> ADC1_FILTER_RESET_W<1> {
        ADC1_FILTER_RESET_W::new(self)
    }
    #[doc = "Bits 16:22 - Set filter factor for DIG ADC2 CRTL."]
    #[inline(always)]
    #[must_use]
    pub fn adc2_filter_factor(&mut self) -> ADC2_FILTER_FACTOR_W<16> {
        ADC2_FILTER_FACTOR_W::new(self)
    }
    #[doc = "Bits 23:29 - Set filter factor for DIG ADC1 CRTL."]
    #[inline(always)]
    #[must_use]
    pub fn adc1_filter_factor(&mut self) -> ADC1_FILTER_FACTOR_W<23> {
        ADC1_FILTER_FACTOR_W::new(self)
    }
    #[doc = "Bit 30 - Enable DIG ADC2 CRTL filter."]
    #[inline(always)]
    #[must_use]
    pub fn adc2_filter_en(&mut self) -> ADC2_FILTER_EN_W<30> {
        ADC2_FILTER_EN_W::new(self)
    }
    #[doc = "Bit 31 - Enable DIG ADC1 CRTL filter."]
    #[inline(always)]
    #[must_use]
    pub fn adc1_filter_en(&mut self) -> ADC1_FILTER_EN_W<31> {
        ADC1_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure the settings of DIG ADC2 filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter_ctrl](index.html) module"]
pub struct FILTER_CTRL_SPEC;
impl crate::RegisterSpec for FILTER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter_ctrl::R](R) reader structure"]
impl crate::Readable for FILTER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter_ctrl::W](W) writer structure"]
impl crate::Writable for FILTER_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER_CTRL to value 0x2040_0000"]
impl crate::Resettable for FILTER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2040_0000;
}
