#[doc = "Register `FEED` writer"]
pub struct W(crate::W<FEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEED_SPEC>;
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
impl From<crate::W<FEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_WDT_FEED` writer - need_des"]
pub type RTC_WDT_FEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEED_SPEC, bool, O>;
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_wdt_feed(&mut self) -> RTC_WDT_FEED_W<31> {
        RTC_WDT_FEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feed](index.html) module"]
pub struct FEED_SPEC;
impl crate::RegisterSpec for FEED_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [feed::W](W) writer structure"]
impl crate::Writable for FEED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEED to value 0"]
impl crate::Resettable for FEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
