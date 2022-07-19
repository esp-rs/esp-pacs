#[doc = "Register `RTC_WDTFEED` reader"]
pub struct R(crate::R<RTC_WDTFEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_WDTFEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_WDTFEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_WDTFEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_WDTFEED` writer"]
pub struct W(crate::W<RTC_WDTFEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_WDTFEED_SPEC>;
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
impl From<crate::W<RTC_WDTFEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_WDTFEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_WDT_FEED` reader - Need add desc"]
pub type RTC_WDT_FEED_R = crate::BitReader<bool>;
#[doc = "Field `RTC_WDT_FEED` writer - Need add desc"]
pub type RTC_WDT_FEED_W<'a> = crate::BitWriter<'a, u32, RTC_WDTFEED_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_wdt_feed(&self) -> RTC_WDT_FEED_R {
        RTC_WDT_FEED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_wdt_feed(&mut self) -> RTC_WDT_FEED_W {
        RTC_WDT_FEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_wdtfeed](index.html) module"]
pub struct RTC_WDTFEED_SPEC;
impl crate::RegisterSpec for RTC_WDTFEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_wdtfeed::R](R) reader structure"]
impl crate::Readable for RTC_WDTFEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_wdtfeed::W](W) writer structure"]
impl crate::Writable for RTC_WDTFEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_WDTFEED to value 0"]
impl crate::Resettable for RTC_WDTFEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
