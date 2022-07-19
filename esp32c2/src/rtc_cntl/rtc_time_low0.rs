#[doc = "Register `RTC_TIME_LOW0` reader"]
pub struct R(crate::R<RTC_TIME_LOW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_LOW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIME_LOW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIME_LOW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIME_LOW0` writer"]
pub struct W(crate::W<RTC_TIME_LOW0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIME_LOW0_SPEC>;
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
impl From<crate::W<RTC_TIME_LOW0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIME_LOW0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_TIMER_VALUE0_LOW` reader - RTC timer low 32 bits"]
pub type RTC_TIMER_VALUE0_LOW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_TIMER_VALUE0_LOW` writer - RTC timer low 32 bits"]
pub type RTC_TIMER_VALUE0_LOW_W<'a> =
    crate::FieldWriter<'a, u32, RTC_TIME_LOW0_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    pub fn rtc_timer_value0_low(&self) -> RTC_TIMER_VALUE0_LOW_R {
        RTC_TIMER_VALUE0_LOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    pub fn rtc_timer_value0_low(&mut self) -> RTC_TIMER_VALUE0_LOW_W {
        RTC_TIMER_VALUE0_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_low0](index.html) module"]
pub struct RTC_TIME_LOW0_SPEC;
impl crate::RegisterSpec for RTC_TIME_LOW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time_low0::R](R) reader structure"]
impl crate::Readable for RTC_TIME_LOW0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_time_low0::W](W) writer structure"]
impl crate::Writable for RTC_TIME_LOW0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIME_LOW0 to value 0"]
impl crate::Resettable for RTC_TIME_LOW0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
