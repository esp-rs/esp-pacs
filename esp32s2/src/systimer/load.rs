#[doc = "Register `LOAD` writer"]
pub type W = crate::W<LOAD_SPEC>;
#[doc = "Field `TIMER_LOAD` writer - Set this bit to 1, the value stored in SYSTIMER_TIMER_LOAD_HI and in SYSTIMER_TIMER_LOAD_LO will be loaded to system timer"]
pub type TIMER_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to 1, the value stored in SYSTIMER_TIMER_LOAD_HI and in SYSTIMER_TIMER_LOAD_LO will be loaded to system timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer_load(&mut self) -> TIMER_LOAD_W<LOAD_SPEC> {
        TIMER_LOAD_W::new(self, 31)
    }
}
#[doc = "Load value to system timer\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOAD_SPEC;
impl crate::RegisterSpec for LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`load::W`](W) writer structure"]
impl crate::Writable for LOAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOAD to value 0"]
impl crate::Resettable for LOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
