#[doc = "Register `UNIT1_LOAD` writer"]
pub type W = crate::W<UNIT1_LOAD_SPEC>;
#[doc = "Field `TIMER_UNIT1_LOAD` writer - Configures whether or not to reload the value of UNIT1, i.e., reload the values of SYSTIMER_TIMER_UNIT1_VALUE_HI and SYSTIMER_TIMER_UNIT1_VALUE_LO to UNIT1. \\\\ 0: No effect \\\\ 1: Reload the value of UNIT1\\\\"]
pub type TIMER_UNIT1_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT1_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to reload the value of UNIT1, i.e., reload the values of SYSTIMER_TIMER_UNIT1_VALUE_HI and SYSTIMER_TIMER_UNIT1_VALUE_LO to UNIT1. \\\\ 0: No effect \\\\ 1: Reload the value of UNIT1\\\\"]
    #[inline(always)]
    pub fn timer_unit1_load(&mut self) -> TIMER_UNIT1_LOAD_W<UNIT1_LOAD_SPEC> {
        TIMER_UNIT1_LOAD_W::new(self, 0)
    }
}
#[doc = "UNIT1 synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit1_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT1_LOAD_SPEC;
impl crate::RegisterSpec for UNIT1_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unit1_load::W`](W) writer structure"]
impl crate::Writable for UNIT1_LOAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNIT1_LOAD to value 0"]
impl crate::Resettable for UNIT1_LOAD_SPEC {}
