#[doc = "Register `COMP2_LOAD` writer"]
pub type W = crate::W<COMP2_LOAD_SPEC>;
#[doc = "Field `TIMER_COMP2_LOAD` writer - Configures whether or not to enable COMP2 synchronization, i.e., reload the alarm value/period to COMP2.\\\\ 0: No effect \\\\ 1: Enable COMP2 synchronization\\\\"]
pub type TIMER_COMP2_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMP2_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable COMP2 synchronization, i.e., reload the alarm value/period to COMP2.\\\\ 0: No effect \\\\ 1: Enable COMP2 synchronization\\\\"]
    #[inline(always)]
    pub fn timer_comp2_load(&mut self) -> TIMER_COMP2_LOAD_W<COMP2_LOAD_SPEC> {
        TIMER_COMP2_LOAD_W::new(self, 0)
    }
}
#[doc = "COMP2 synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP2_LOAD_SPEC;
impl crate::RegisterSpec for COMP2_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`comp2_load::W`](W) writer structure"]
impl crate::Writable for COMP2_LOAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP2_LOAD to value 0"]
impl crate::Resettable for COMP2_LOAD_SPEC {}
