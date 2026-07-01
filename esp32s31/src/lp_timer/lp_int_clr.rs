#[doc = "Register `LP_INT_CLR` writer"]
pub type W = crate::W<LP_INT_CLR_SPEC>;
#[doc = "Field `TICK` writer - need_des"]
pub type TICK_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UPDTAE_DONE` writer - need_des"]
pub type UPDTAE_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MAIN_TIMER_OVERFLOW` writer - need_des"]
pub type MAIN_TIMER_OVERFLOW_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MAIN_TIMER` writer - need_des"]
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tick(&mut self) -> TICK_W<'_, LP_INT_CLR_SPEC> {
        TICK_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn updtae_done(&mut self) -> UPDTAE_DONE_W<'_, LP_INT_CLR_SPEC> {
        UPDTAE_DONE_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow(&mut self) -> MAIN_TIMER_OVERFLOW_W<'_, LP_INT_CLR_SPEC> {
        MAIN_TIMER_OVERFLOW_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<'_, LP_INT_CLR_SPEC> {
        MAIN_TIMER_W::new(self, 31)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf000_0000;
}
#[doc = "`reset()` method sets LP_INT_CLR to value 0"]
impl crate::Resettable for LP_INT_CLR_SPEC {}
