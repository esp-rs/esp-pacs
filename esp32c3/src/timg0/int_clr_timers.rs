#[doc = "Register `INT_CLR_TIMERS` writer"]
pub type W = crate::W<INT_CLR_TIMERS_SPEC>;
#[doc = "Field `T0_INT_CLR` writer - t0_int_clr"]
pub type T0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_CLR` writer - wdt_int_clr"]
pub type WDT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - t0_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn t0_int_clr(&mut self) -> T0_INT_CLR_W<INT_CLR_TIMERS_SPEC> {
        T0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - wdt_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<INT_CLR_TIMERS_SPEC> {
        WDT_INT_CLR_W::new(self, 1)
    }
}
#[doc = "INT_CLR_TIMG_REG\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_TIMERS_SPEC;
impl crate::RegisterSpec for INT_CLR_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr_timers::W`](W) writer structure"]
impl crate::Writable for INT_CLR_TIMERS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR_TIMERS to value 0"]
impl crate::Resettable for INT_CLR_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
