#[doc = "Register `FILTER_STATUS` reader"]
pub type R = crate::R<FILTER_STATUS_SPEC>;
#[doc = "Field `ADC2_FILTER_DATA` reader - ADC2 filter data."]
pub type ADC2_FILTER_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `ADC1_FILTER_DATA` reader - ADC1 filter data."]
pub type ADC1_FILTER_DATA_R = crate::FieldReader<u16>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_STATUS")
            .field("adc2_filter_data", &self.adc2_filter_data().bits())
            .field("adc1_filter_data", &self.adc1_filter_data().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FILTER_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Data status of DIG ADC2 filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_STATUS_SPEC;
impl crate::RegisterSpec for FILTER_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_status::R`](R) reader structure"]
impl crate::Readable for FILTER_STATUS_SPEC {}
#[doc = "`reset()` method sets FILTER_STATUS to value 0"]
impl crate::Resettable for FILTER_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
