#[doc = "Register `WDTWPROTECT` reader"]
pub type R = crate::R<WDTWPROTECT_SPEC>;
#[doc = "Register `WDTWPROTECT` writer"]
pub type W = crate::W<WDTWPROTECT_SPEC>;
#[doc = "Field `WDT_WKEY` reader - the key of rtc wdt"]
pub type WDT_WKEY_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_WKEY` writer - the key of rtc wdt"]
pub type WDT_WKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the key of rtc wdt"]
    #[inline(always)]
    pub fn wdt_wkey(&self) -> WDT_WKEY_R {
        WDT_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTWPROTECT")
            .field("wdt_wkey", &format_args!("{}", self.wdt_wkey().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTWPROTECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the key of rtc wdt"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_wkey(&mut self) -> WDT_WKEY_W<WDTWPROTECT_SPEC> {
        WDT_WKEY_W::new(self, 0)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTWPROTECT_SPEC;
impl crate::RegisterSpec for WDTWPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtwprotect::R`](R) reader structure"]
impl crate::Readable for WDTWPROTECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtwprotect::W`](W) writer structure"]
impl crate::Writable for WDTWPROTECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTWPROTECT to value 0"]
impl crate::Resettable for WDTWPROTECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
