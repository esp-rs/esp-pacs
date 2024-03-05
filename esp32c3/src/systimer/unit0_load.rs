#[doc = "Register `UNIT0_LOAD` writer"]
pub type W = crate::W<UNIT0_LOAD_SPEC>;
#[doc = "Field `TIMER_UNIT0_LOAD` writer - timer unit0 load value"]
pub type TIMER_UNIT0_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT0_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - timer unit0 load value"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit0_load(&mut self) -> TIMER_UNIT0_LOAD_W<UNIT0_LOAD_SPEC> {
        TIMER_UNIT0_LOAD_W::new(self, 0)
    }
}
#[doc = "SYSTIMER_UNIT0_LOAD.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit0_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT0_LOAD_SPEC;
impl crate::RegisterSpec for UNIT0_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unit0_load::W`](W) writer structure"]
impl crate::Writable for UNIT0_LOAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UNIT0_LOAD to value 0"]
impl crate::Resettable for UNIT0_LOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
