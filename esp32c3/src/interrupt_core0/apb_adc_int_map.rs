#[doc = "Register `APB_ADC_INT_MAP` reader"]
pub struct R(crate::R<APB_ADC_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_ADC_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_ADC_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_ADC_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_ADC_INT_MAP` writer"]
pub struct W(crate::W<APB_ADC_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_ADC_INT_MAP_SPEC>;
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
impl From<crate::W<APB_ADC_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_ADC_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_ADC_INT_MAP` reader - reg_core0_apb_adc_int_map"]
pub type APB_ADC_INT_MAP_R = crate::FieldReader;
#[doc = "Field `APB_ADC_INT_MAP` writer - reg_core0_apb_adc_int_map"]
pub type APB_ADC_INT_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, APB_ADC_INT_MAP_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_apb_adc_int_map"]
    #[inline(always)]
    pub fn apb_adc_int_map(&self) -> APB_ADC_INT_MAP_R {
        APB_ADC_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_ADC_INT_MAP")
            .field(
                "apb_adc_int_map",
                &format_args!("{}", self.apb_adc_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_ADC_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_apb_adc_int_map"]
    #[inline(always)]
    #[must_use]
    pub fn apb_adc_int_map(&mut self) -> APB_ADC_INT_MAP_W<0> {
        APB_ADC_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adc intr map register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_adc_int_map](index.html) module"]
pub struct APB_ADC_INT_MAP_SPEC;
impl crate::RegisterSpec for APB_ADC_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_adc_int_map::R](R) reader structure"]
impl crate::Readable for APB_ADC_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_adc_int_map::W](W) writer structure"]
impl crate::Writable for APB_ADC_INT_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_ADC_INT_MAP to value 0"]
impl crate::Resettable for APB_ADC_INT_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
