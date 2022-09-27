#[doc = "Register `TIME_LOW1` reader"]
pub struct R(crate::R<TIME_LOW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_LOW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_LOW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_LOW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC_TIMER_VALUE1_LOW` reader - RTC timer low 32 bits"]
pub type RTC_TIMER_VALUE1_LOW_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    pub fn rtc_timer_value1_low(&self) -> RTC_TIMER_VALUE1_LOW_R {
        RTC_TIMER_VALUE1_LOW_R::new(self.bits)
    }
}
#[doc = "RTC timer low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_low1](index.html) module"]
pub struct TIME_LOW1_SPEC;
impl crate::RegisterSpec for TIME_LOW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_low1::R](R) reader structure"]
impl crate::Readable for TIME_LOW1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIME_LOW1 to value 0"]
impl crate::Resettable for TIME_LOW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
