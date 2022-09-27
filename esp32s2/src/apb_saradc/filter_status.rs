#[doc = "Register `FILTER_STATUS` reader"]
pub struct R(crate::R<FILTER_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC2_FILTER_DATA` reader - ADC2 filter data."]
pub type ADC2_FILTER_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC1_FILTER_DATA` reader - ADC1 filter data."]
pub type ADC1_FILTER_DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC2 filter data."]
    #[inline(always)]
    pub fn adc2_filter_data(&self) -> ADC2_FILTER_DATA_R {
        ADC2_FILTER_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADC1 filter data."]
    #[inline(always)]
    pub fn adc1_filter_data(&self) -> ADC1_FILTER_DATA_R {
        ADC1_FILTER_DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Data status of DIG ADC2 filter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter_status](index.html) module"]
pub struct FILTER_STATUS_SPEC;
impl crate::RegisterSpec for FILTER_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter_status::R](R) reader structure"]
impl crate::Readable for FILTER_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FILTER_STATUS to value 0"]
impl crate::Resettable for FILTER_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
