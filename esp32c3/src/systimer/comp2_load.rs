#[doc = "Register `COMP2_LOAD` writer"]
pub type W = crate::W<COMP2_LOAD_SPEC>;
#[doc = "Field `TIMER_COMP2_LOAD` writer - timer comp2 load value"]
pub type TIMER_COMP2_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMP2_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - timer comp2 load value"]
    #[inline(always)]
    #[must_use]
    pub fn timer_comp2_load(&mut self) -> TIMER_COMP2_LOAD_W<COMP2_LOAD_SPEC> {
        TIMER_COMP2_LOAD_W::new(self, 0)
    }
}
#[doc = "SYSTIMER_COMP2_LOAD.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP2_LOAD_SPEC;
impl crate::RegisterSpec for COMP2_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`comp2_load::W`](W) writer structure"]
impl crate::Writable for COMP2_LOAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP2_LOAD to value 0"]
impl crate::Resettable for COMP2_LOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
