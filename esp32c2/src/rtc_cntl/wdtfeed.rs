#[doc = "Register `WDTFEED` reader"]
pub type R = crate::R<WDTFEED_SPEC>;
#[doc = "Register `WDTFEED` writer"]
pub type W = crate::W<WDTFEED_SPEC>;
#[doc = "Field `WDT_FEED` reader - Need add desc"]
pub type WDT_FEED_R = crate::BitReader;
#[doc = "Field `WDT_FEED` writer - Need add desc"]
pub type WDT_FEED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn wdt_feed(&self) -> WDT_FEED_R {
        WDT_FEED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTFEED")
            .field("wdt_feed", &format_args!("{}", self.wdt_feed().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTFEED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_feed(&mut self) -> WDT_FEED_W<WDTFEED_SPEC, 31> {
        WDT_FEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtfeed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTFEED_SPEC;
impl crate::RegisterSpec for WDTFEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtfeed::R`](R) reader structure"]
impl crate::Readable for WDTFEED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtfeed::W`](W) writer structure"]
impl crate::Writable for WDTFEED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTFEED to value 0"]
impl crate::Resettable for WDTFEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
