#[doc = "Register `RTC_TIME_HIGH1` reader"]
pub struct R(crate::R<RTC_TIME_HIGH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_HIGH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIME_HIGH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIME_HIGH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIME_HIGH1` writer"]
pub struct W(crate::W<RTC_TIME_HIGH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIME_HIGH1_SPEC>;
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
impl From<crate::W<RTC_TIME_HIGH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIME_HIGH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_TIMER_VALUE1_HIGH` reader - RTC timer high 16 bits"]
pub type RTC_TIMER_VALUE1_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTC_TIMER_VALUE1_HIGH` writer - RTC timer high 16 bits"]
pub type RTC_TIMER_VALUE1_HIGH_W<'a> =
    crate::FieldWriter<'a, u32, RTC_TIME_HIGH1_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn rtc_timer_value1_high(&self) -> RTC_TIMER_VALUE1_HIGH_R {
        RTC_TIMER_VALUE1_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn rtc_timer_value1_high(&mut self) -> RTC_TIMER_VALUE1_HIGH_W {
        RTC_TIMER_VALUE1_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_high1](index.html) module"]
pub struct RTC_TIME_HIGH1_SPEC;
impl crate::RegisterSpec for RTC_TIME_HIGH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time_high1::R](R) reader structure"]
impl crate::Readable for RTC_TIME_HIGH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_time_high1::W](W) writer structure"]
impl crate::Writable for RTC_TIME_HIGH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIME_HIGH1 to value 0"]
impl crate::Resettable for RTC_TIME_HIGH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
