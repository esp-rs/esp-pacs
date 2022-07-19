#[doc = "Register `RTC_PWC` reader"]
pub struct R(crate::R<RTC_PWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_PWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_PWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_PWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_PWC` writer"]
pub struct W(crate::W<RTC_PWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_PWC_SPEC>;
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
impl From<crate::W<RTC_PWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_PWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_PAD_FORCE_HOLD` reader - rtc pad force hold"]
pub type RTC_PAD_FORCE_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_PAD_FORCE_HOLD` writer - rtc pad force hold"]
pub type RTC_PAD_FORCE_HOLD_W<'a> = crate::BitWriter<'a, u32, RTC_PWC_SPEC, bool, 21>;
impl R {
    #[doc = "Bit 21 - rtc pad force hold"]
    #[inline(always)]
    pub fn rtc_pad_force_hold(&self) -> RTC_PAD_FORCE_HOLD_R {
        RTC_PAD_FORCE_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - rtc pad force hold"]
    #[inline(always)]
    pub fn rtc_pad_force_hold(&mut self) -> RTC_PAD_FORCE_HOLD_W {
        RTC_PAD_FORCE_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_pwc](index.html) module"]
pub struct RTC_PWC_SPEC;
impl crate::RegisterSpec for RTC_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_pwc::R](R) reader structure"]
impl crate::Readable for RTC_PWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_pwc::W](W) writer structure"]
impl crate::Writable for RTC_PWC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_PWC to value 0"]
impl crate::Resettable for RTC_PWC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
