#[doc = "Register `LP_INT_RAW` reader"]
pub type R = crate::R<LP_INT_RAW_SPEC>;
#[doc = "Register `LP_INT_RAW` writer"]
pub type W = crate::W<LP_INT_RAW_SPEC>;
#[doc = "Field `TICK_LP_INT_RAW` reader - need_des"]
pub type TICK_LP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TICK_LP_INT_RAW` writer - need_des"]
pub type TICK_LP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDTAE_DONE_LP_INT_RAW` reader - need_des"]
pub type UPDTAE_DONE_LP_INT_RAW_R = crate::BitReader;
#[doc = "Field `UPDTAE_DONE_LP_INT_RAW` writer - need_des"]
pub type UPDTAE_DONE_LP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_RAW` reader - need_des"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_RAW_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_RAW` writer - need_des"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_LP_INT_RAW` reader - need_des"]
pub type MAIN_TIMER_LP_INT_RAW_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_LP_INT_RAW` writer - need_des"]
pub type MAIN_TIMER_LP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tick_lp_int_raw(&self) -> TICK_LP_INT_RAW_R {
        TICK_LP_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn updtae_done_lp_int_raw(&self) -> UPDTAE_DONE_LP_INT_RAW_R {
        UPDTAE_DONE_LP_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_raw(&self) -> MAIN_TIMER_OVERFLOW_LP_INT_RAW_R {
        MAIN_TIMER_OVERFLOW_LP_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_lp_int_raw(&self) -> MAIN_TIMER_LP_INT_RAW_R {
        MAIN_TIMER_LP_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_RAW")
            .field("tick_lp_int_raw", &self.tick_lp_int_raw())
            .field("updtae_done_lp_int_raw", &self.updtae_done_lp_int_raw())
            .field(
                "main_timer_overflow_lp_int_raw",
                &self.main_timer_overflow_lp_int_raw(),
            )
            .field("main_timer_lp_int_raw", &self.main_timer_lp_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tick_lp_int_raw(&mut self) -> TICK_LP_INT_RAW_W<'_, LP_INT_RAW_SPEC> {
        TICK_LP_INT_RAW_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn updtae_done_lp_int_raw(&mut self) -> UPDTAE_DONE_LP_INT_RAW_W<'_, LP_INT_RAW_SPEC> {
        UPDTAE_DONE_LP_INT_RAW_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_raw(
        &mut self,
    ) -> MAIN_TIMER_OVERFLOW_LP_INT_RAW_W<'_, LP_INT_RAW_SPEC> {
        MAIN_TIMER_OVERFLOW_LP_INT_RAW_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_lp_int_raw(&mut self) -> MAIN_TIMER_LP_INT_RAW_W<'_, LP_INT_RAW_SPEC> {
        MAIN_TIMER_LP_INT_RAW_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_RAW_SPEC;
impl crate::RegisterSpec for LP_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_raw::R`](R) reader structure"]
impl crate::Readable for LP_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_int_raw::W`](W) writer structure"]
impl crate::Writable for LP_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INT_RAW to value 0"]
impl crate::Resettable for LP_INT_RAW_SPEC {}
