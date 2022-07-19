#[doc = "Register `RTC_SLP_WAKEUP_CAUSE` reader"]
pub struct R(crate::R<RTC_SLP_WAKEUP_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SLP_WAKEUP_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SLP_WAKEUP_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SLP_WAKEUP_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SLP_WAKEUP_CAUSE` writer"]
pub struct W(crate::W<RTC_SLP_WAKEUP_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SLP_WAKEUP_CAUSE_SPEC>;
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
impl From<crate::W<RTC_SLP_WAKEUP_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SLP_WAKEUP_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_CAUSE` reader - sleep wakeup cause"]
pub type WAKEUP_CAUSE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WAKEUP_CAUSE` writer - sleep wakeup cause"]
pub type WAKEUP_CAUSE_W<'a> =
    crate::FieldWriter<'a, u32, RTC_SLP_WAKEUP_CAUSE_SPEC, u32, u32, 17, 0>;
impl R {
    #[doc = "Bits 0:16 - sleep wakeup cause"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - sleep wakeup cause"]
    #[inline(always)]
    pub fn wakeup_cause(&mut self) -> WAKEUP_CAUSE_W {
        WAKEUP_CAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_slp_wakeup_cause](index.html) module"]
pub struct RTC_SLP_WAKEUP_CAUSE_SPEC;
impl crate::RegisterSpec for RTC_SLP_WAKEUP_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_slp_wakeup_cause::R](R) reader structure"]
impl crate::Readable for RTC_SLP_WAKEUP_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_slp_wakeup_cause::W](W) writer structure"]
impl crate::Writable for RTC_SLP_WAKEUP_CAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SLP_WAKEUP_CAUSE to value 0"]
impl crate::Resettable for RTC_SLP_WAKEUP_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
