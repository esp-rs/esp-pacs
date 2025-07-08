#[doc = "Register `FEED` writer"]
pub type W = crate::W<FEED_SPEC>;
#[doc = "Field `RTC_WDT_FEED` writer - Configure this bit to feed the RWDT.\\\\ 0: Invalid\\\\ 1: Feed RWDT"]
pub type RTC_WDT_FEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FEED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - Configure this bit to feed the RWDT.\\\\ 0: Invalid\\\\ 1: Feed RWDT"]
    #[inline(always)]
    pub fn rtc_wdt_feed(&mut self) -> RTC_WDT_FEED_W<FEED_SPEC> {
        RTC_WDT_FEED_W::new(self, 31)
    }
}
#[doc = "Configure the feed function of RWDT\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FEED_SPEC;
impl crate::RegisterSpec for FEED_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`feed::W`](W) writer structure"]
impl crate::Writable for FEED_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FEED to value 0"]
impl crate::Resettable for FEED_SPEC {}
