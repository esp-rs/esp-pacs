#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC2_THRES_INT_ENA` reader - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
pub type ADC2_THRES_INT_ENA_R = crate::BitReader;
#[doc = "Field `ADC2_THRES_INT_ENA` writer - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
pub type ADC2_THRES_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `ADC1_THRES_INT_ENA` reader - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
pub type ADC1_THRES_INT_ENA_R = crate::BitReader;
#[doc = "Field `ADC1_THRES_INT_ENA` writer - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
pub type ADC1_THRES_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `ADC2_DONE_INT_ENA` reader - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
pub type ADC2_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `ADC2_DONE_INT_ENA` writer - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
pub type ADC2_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `ADC1_DONE_INT_ENA` reader - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
pub type ADC1_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `ADC1_DONE_INT_ENA` writer - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
pub type ADC1_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 28 - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc2_thres_int_ena(&self) -> ADC2_THRES_INT_ENA_R {
        ADC2_THRES_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc1_thres_int_ena(&self) -> ADC1_THRES_INT_ENA_R {
        ADC1_THRES_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc2_done_int_ena(&self) -> ADC2_DONE_INT_ENA_R {
        ADC2_DONE_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc1_done_int_ena(&self) -> ADC1_DONE_INT_ENA_R {
        ADC1_DONE_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "adc2_thres_int_ena",
                &format_args!("{}", self.adc2_thres_int_ena().bit()),
            )
            .field(
                "adc1_thres_int_ena",
                &format_args!("{}", self.adc1_thres_int_ena().bit()),
            )
            .field(
                "adc2_done_int_ena",
                &format_args!("{}", self.adc2_done_int_ena().bit()),
            )
            .field(
                "adc1_done_int_ena",
                &format_args!("{}", self.adc1_done_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 28 - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adc2_thres_int_ena(&mut self) -> ADC2_THRES_INT_ENA_W<28> {
        ADC2_THRES_INT_ENA_W::new(self)
    }
    #[doc = "Bit 29 - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adc1_thres_int_ena(&mut self) -> ADC1_THRES_INT_ENA_W<29> {
        ADC1_THRES_INT_ENA_W::new(self)
    }
    #[doc = "Bit 30 - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adc2_done_int_ena(&mut self) -> ADC2_DONE_INT_ENA_W<30> {
        ADC2_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 31 - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adc1_done_int_ena(&mut self) -> ADC1_DONE_INT_ENA_W<31> {
        ADC1_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable DIG ADC interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
