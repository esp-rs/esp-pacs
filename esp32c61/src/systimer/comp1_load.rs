#[doc = "Register `COMP1_LOAD` writer"]
pub type W = crate::W<COMP1_LOAD_SPEC>;
#[doc = "Field `TIMER_COMP1_LOAD` writer - Configures whether or not to enable COMP1 synchronization, i.e., reload the alarm value/period to COMP1. \\\\ 0: No effect \\\\ 1: Enable COMP1 synchronization\\\\"]
pub type TIMER_COMP1_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMP1_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable COMP1 synchronization, i.e., reload the alarm value/period to COMP1. \\\\ 0: No effect \\\\ 1: Enable COMP1 synchronization\\\\"]
    #[inline(always)]
    pub fn timer_comp1_load(&mut self) -> TIMER_COMP1_LOAD_W<COMP1_LOAD_SPEC> {
        TIMER_COMP1_LOAD_W::new(self, 0)
    }
}
#[doc = "COMP1 synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP1_LOAD_SPEC;
impl crate::RegisterSpec for COMP1_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`comp1_load::W`](W) writer structure"]
impl crate::Writable for COMP1_LOAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP1_LOAD to value 0"]
impl crate::Resettable for COMP1_LOAD_SPEC {}
