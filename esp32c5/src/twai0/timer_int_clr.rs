#[doc = "Register `TIMER_INT_CLR` writer"]
pub type W = crate::W<TIMER_INT_CLR_SPEC>;
#[doc = "Field `TIMER_OVERFLOW_INT_CLR` writer - The clear signal for read_done interrupt."]
pub type TIMER_OVERFLOW_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for read_done interrupt."]
    #[inline(always)]
    pub fn timer_overflow_int_clr(&mut self) -> TIMER_OVERFLOW_INT_CLR_W<TIMER_INT_CLR_SPEC> {
        TIMER_OVERFLOW_INT_CLR_W::new(self, 0)
    }
}
#[doc = "TWAIFD interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_INT_CLR_SPEC;
impl crate::RegisterSpec for TIMER_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timer_int_clr::W`](W) writer structure"]
impl crate::Writable for TIMER_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_INT_CLR to value 0"]
impl crate::Resettable for TIMER_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
