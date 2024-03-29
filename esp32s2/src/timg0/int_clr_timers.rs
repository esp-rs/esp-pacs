#[doc = "Register `INT_CLR_TIMERS` writer"]
pub type W = crate::W<INT_CLR_TIMERS_SPEC>;
#[doc = "Field `T0` writer - Set this bit to clear the TIMG_T0_INT interrupt."]
pub type T0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `T1` writer - Set this bit to clear the TIMG_T1_INT interrupt."]
pub type T1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WDT` writer - Set this bit to clear the TIMG_WDT_INT interrupt."]
pub type WDT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LACT` writer - Set this bit to clear the TIMG_LACT_INT interrupt."]
pub type LACT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the TIMG_T0_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn t0(&mut self) -> T0_W<INT_CLR_TIMERS_SPEC> {
        T0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the TIMG_T1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn t1(&mut self) -> T1_W<INT_CLR_TIMERS_SPEC> {
        T1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<INT_CLR_TIMERS_SPEC> {
        WDT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the TIMG_LACT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lact(&mut self) -> LACT_W<INT_CLR_TIMERS_SPEC> {
        LACT_W::new(self, 3)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_TIMERS_SPEC;
impl crate::RegisterSpec for INT_CLR_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr_timers::W`](W) writer structure"]
impl crate::Writable for INT_CLR_TIMERS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets INT_CLR_TIMERS to value 0"]
impl crate::Resettable for INT_CLR_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
