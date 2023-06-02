#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC2_THRES_INT_ST` reader - Status of APB_SARADC_ADC2_THRES_INT interrupt."]
pub type ADC2_THRES_INT_ST_R = crate::BitReader;
#[doc = "Field `ADC1_THRES_INT_ST` reader - Status of APB_SARADC_ADC1_THRES_INT interrupt."]
pub type ADC1_THRES_INT_ST_R = crate::BitReader;
#[doc = "Field `ADC2_DONE_INT_ST` reader - Status of APB_SARADC_ADC2_DONE_INT interrupt."]
pub type ADC2_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `ADC1_DONE_INT_ST` reader - Status of APB_SARADC_ADC1_DONE_INT interrupt."]
pub type ADC1_DONE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 28 - Status of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc2_thres_int_st(&self) -> ADC2_THRES_INT_ST_R {
        ADC2_THRES_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Status of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc1_thres_int_st(&self) -> ADC1_THRES_INT_ST_R {
        ADC1_THRES_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Status of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc2_done_int_st(&self) -> ADC2_DONE_INT_ST_R {
        ADC2_DONE_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Status of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc1_done_int_st(&self) -> ADC1_DONE_INT_ST_R {
        ADC1_DONE_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "adc2_thres_int_st",
                &format_args!("{}", self.adc2_thres_int_st().bit()),
            )
            .field(
                "adc1_thres_int_st",
                &format_args!("{}", self.adc1_thres_int_st().bit()),
            )
            .field(
                "adc2_done_int_st",
                &format_args!("{}", self.adc2_done_int_st().bit()),
            )
            .field(
                "adc1_done_int_st",
                &format_args!("{}", self.adc1_done_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DIG ADC interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
