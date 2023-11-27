#[doc = "Register `LP_INT_ST` reader"]
pub type R = crate::R<LP_INT_ST_SPEC>;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_ST` reader - need_des"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_ST_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_LP_INT_ST` reader - need_des"]
pub type MAIN_TIMER_LP_INT_ST_R = crate::BitReader;
impl R {
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
            .field(
                "main_timer_overflow_lp_int_st",
                &format_args!("{}", self.main_timer_overflow_lp_int_st().bit()),
            )
            .field(
                "main_timer_lp_int_st",
                &format_args!("{}", self.main_timer_lp_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_st::R`](R) reader structure"]
impl crate::Readable for LP_INT_ST_SPEC {}
#[doc = "`reset()` method sets LP_INT_ST to value 0"]
impl crate::Resettable for LP_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
