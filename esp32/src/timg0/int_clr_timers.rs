#[doc = "Register `INT_CLR_TIMERS` writer"]
pub type W = crate::W<INT_CLR_TIMERS_SPEC>;
#[doc = "Field `T0_INT_CLR` writer - interrupt when timer0 alarm"]
pub type T0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T1_INT_CLR` writer - interrupt when timer1 alarm"]
pub type T1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_CLR` writer - Interrupt when an interrupt stage timeout"]
pub type WDT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_INT_CLR` writer - "]
pub type LACT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    #[must_use]
    pub fn t0_int_clr(&mut self) -> T0_INT_CLR_W<INT_CLR_TIMERS_SPEC> {
        T0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    #[must_use]
    pub fn t1_int_clr(&mut self) -> T1_INT_CLR_W<INT_CLR_TIMERS_SPEC> {
        T1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<INT_CLR_TIMERS_SPEC> {
        WDT_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn lact_int_clr(&mut self) -> LACT_INT_CLR_W<INT_CLR_TIMERS_SPEC> {
        LACT_INT_CLR_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
