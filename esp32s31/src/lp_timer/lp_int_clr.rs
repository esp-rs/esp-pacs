#[doc = "Register `LP_INT_CLR` writer"]
pub type W = crate::W<LP_INT_CLR_SPEC>;
#[doc = "Field `TICK_LP_INT_CLR` writer - need_des"]
pub type TICK_LP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDTAE_DONE_LP_INT_CLR` writer - need_des"]
pub type UPDTAE_DONE_LP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tick_lp_int_clr(&mut self) -> TICK_LP_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        TICK_LP_INT_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn updtae_done_lp_int_clr(&mut self) -> UPDTAE_DONE_LP_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        UPDTAE_DONE_LP_INT_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_clr(
        &mut self,
    ) -> MAIN_TIMER_OVERFLOW_LP_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        MAIN_TIMER_OVERFLOW_LP_INT_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_lp_int_clr(&mut self) -> MAIN_TIMER_LP_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        MAIN_TIMER_LP_INT_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_CLR_SPEC;
impl crate::RegisterSpec for LP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lp_int_clr::W`](W) writer structure"]
impl crate::Writable for LP_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INT_CLR to value 0"]
impl crate::Resettable for LP_INT_CLR_SPEC {}
