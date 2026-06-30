#[doc = "Register `LP_INT_ST` reader"]
pub type R = crate::R<LP_INT_ST_SPEC>;
#[doc = "Field `TICK_LP_INT_ST` reader - need_des"]
pub type TICK_LP_INT_ST_R = crate::BitReader;
#[doc = "Field `UPDTAE_DONE_INT_LP_ST` reader - need_des"]
pub type UPDTAE_DONE_INT_LP_ST_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_ST` reader - need_des"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_ST_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_LP_INT_ST` reader - need_des"]
pub type MAIN_TIMER_LP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tick_lp_int_st(&self) -> TICK_LP_INT_ST_R {
        TICK_LP_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn updtae_done_int_lp_st(&self) -> UPDTAE_DONE_INT_LP_ST_R {
        UPDTAE_DONE_INT_LP_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_st(&self) -> MAIN_TIMER_OVERFLOW_LP_INT_ST_R {
        MAIN_TIMER_OVERFLOW_LP_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_lp_int_st(&self) -> MAIN_TIMER_LP_INT_ST_R {
        MAIN_TIMER_LP_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ST")
            .field("tick_lp_int_st", &self.tick_lp_int_st())
            .field("updtae_done_int_lp_st", &self.updtae_done_int_lp_st())
            .field(
                "main_timer_overflow_lp_int_st",
                &self.main_timer_overflow_lp_int_st(),
            )
            .field("main_timer_lp_int_st", &self.main_timer_lp_int_st())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_st::R`](R) reader structure"]
impl crate::Readable for LP_INT_ST_SPEC {}
#[doc = "`reset()` method sets LP_INT_ST to value 0"]
impl crate::Resettable for LP_INT_ST_SPEC {}
