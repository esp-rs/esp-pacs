#[doc = "Register `LP_INT_CLR` writer"]
pub type W = crate::W<LP_INT_CLR_SPEC>;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_CLR` writer - need_des"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_LP_INT_CLR` writer - need_des"]
pub type MAIN_TIMER_LP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_overflow_lp_int_clr(
        &mut self,
    ) -> MAIN_TIMER_OVERFLOW_LP_INT_CLR_W<LP_INT_CLR_SPEC> {
        MAIN_TIMER_OVERFLOW_LP_INT_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_lp_int_clr(&mut self) -> MAIN_TIMER_LP_INT_CLR_W<LP_INT_CLR_SPEC> {
        MAIN_TIMER_LP_INT_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_CLR_SPEC;
impl crate::RegisterSpec for LP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lp_int_clr::W`](W) writer structure"]
impl crate::Writable for LP_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_INT_CLR to value 0"]
impl crate::Resettable for LP_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
