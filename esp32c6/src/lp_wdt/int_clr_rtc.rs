#[doc = "Register `INT_CLR_RTC` writer"]
pub type W = crate::W<INT_CLR_RTC_SPEC>;
#[doc = "Field `SUPER_WDT_INT_CLR` writer - need_des"]
pub type SUPER_WDT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_CLR` writer - need_des"]
pub type WDT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn super_wdt_int_clr(&mut self) -> SUPER_WDT_INT_CLR_W<INT_CLR_RTC_SPEC> {
        SUPER_WDT_INT_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<INT_CLR_RTC_SPEC> {
        WDT_INT_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_rtc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_RTC_SPEC;
impl crate::RegisterSpec for INT_CLR_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr_rtc::W`](W) writer structure"]
impl crate::Writable for INT_CLR_RTC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR_RTC to value 0"]
impl crate::Resettable for INT_CLR_RTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
