#[doc = "Register `UNIT0_LOAD` writer"]
pub type W = crate::W<UNIT0_LOAD_SPEC>;
#[doc = "Field `TIMER_UNIT0_LOAD` writer - Configures whether or not to reload the value of UNIT0, i.e., reloads the values of SYSTIMER_TIMER_UNIT0_VALUE_HI and SYSTIMER_TIMER_UNIT0_VALUE_LO to UNIT0. \\\\ 0: No effect \\\\ 1: Reload the value of UNIT0\\\\"]
pub type TIMER_UNIT0_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT0_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to reload the value of UNIT0, i.e., reloads the values of SYSTIMER_TIMER_UNIT0_VALUE_HI and SYSTIMER_TIMER_UNIT0_VALUE_LO to UNIT0. \\\\ 0: No effect \\\\ 1: Reload the value of UNIT0\\\\"]
    #[inline(always)]
    pub fn timer_unit0_load(&mut self) -> TIMER_UNIT0_LOAD_W<UNIT0_LOAD_SPEC> {
        TIMER_UNIT0_LOAD_W::new(self, 0)
    }
}
#[doc = "UNIT0 synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit0_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT0_LOAD_SPEC;
impl crate::RegisterSpec for UNIT0_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unit0_load::W`](W) writer structure"]
impl crate::Writable for UNIT0_LOAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNIT0_LOAD to value 0"]
impl crate::Resettable for UNIT0_LOAD_SPEC {}
