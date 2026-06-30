#[doc = "Register `INT_CLR_TIMERS` writer"]
pub type W = crate::W<INT_CLR_TIMERS_SPEC>;
#[doc = "Field `T0_INT_CLR` writer - Write 1 to clear the TIMG_T0_INT interrupt."]
pub type T0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T1_INT_CLR` writer - Write 1 to clear the TIMG_T1_INT interrupt."]
pub type T1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_CLR` writer - Write 1 to clear the TIMG_WDT_INT interrupt."]
pub type WDT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0_int_clr(&mut self) -> T0_INT_CLR_W<'_, INT_CLR_TIMERS_SPEC> {
        T0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1_int_clr(&mut self) -> T1_INT_CLR_W<'_, INT_CLR_TIMERS_SPEC> {
        T1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<'_, INT_CLR_TIMERS_SPEC> {
        WDT_INT_CLR_W::new(self, 2)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr_timers::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_TIMERS_SPEC;
impl crate::RegisterSpec for INT_CLR_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr_timers::W`](W) writer structure"]
impl crate::Writable for INT_CLR_TIMERS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR_TIMERS to value 0"]
impl crate::Resettable for INT_CLR_TIMERS_SPEC {}
