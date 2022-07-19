#[doc = "Register `RTC_CNTL_DATE` reader"]
pub struct R(crate::R<RTC_CNTL_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNTL_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNTL_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNTL_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNTL_DATE` writer"]
pub struct W(crate::W<RTC_CNTL_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNTL_DATE_SPEC>;
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
impl From<crate::W<RTC_CNTL_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNTL_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CNTL_DATE` reader - Need add desc"]
pub type RTC_CNTL_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_CNTL_DATE` writer - Need add desc"]
pub type RTC_CNTL_DATE_W<'a> = crate::FieldWriter<'a, u32, RTC_CNTL_DATE_SPEC, u32, u32, 28, 0>;
impl R {
    #[doc = "Bits 0:27 - Need add desc"]
    #[inline(always)]
    pub fn rtc_cntl_date(&self) -> RTC_CNTL_DATE_R {
        RTC_CNTL_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - Need add desc"]
    #[inline(always)]
    pub fn rtc_cntl_date(&mut self) -> RTC_CNTL_DATE_W {
        RTC_CNTL_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_date](index.html) module"]
pub struct RTC_CNTL_DATE_SPEC;
impl crate::RegisterSpec for RTC_CNTL_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cntl_date::R](R) reader structure"]
impl crate::Readable for RTC_CNTL_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_date::W](W) writer structure"]
impl crate::Writable for RTC_CNTL_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CNTL_DATE to value 0x0210_7190"]
impl crate::Resettable for RTC_CNTL_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210_7190
    }
}
