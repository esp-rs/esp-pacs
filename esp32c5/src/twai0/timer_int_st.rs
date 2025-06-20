#[doc = "Register `TIMER_INT_ST` reader"]
pub type R = crate::R<TIMER_INT_ST_SPEC>;
#[doc = "Field `TIMER_OVERFLOW_INT_ST` reader - The status signal for read_done interrupt."]
pub type TIMER_OVERFLOW_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status signal for read_done interrupt."]
    #[inline(always)]
    pub fn timer_overflow_int_st(&self) -> TIMER_OVERFLOW_INT_ST_R {
        TIMER_OVERFLOW_INT_ST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_INT_ST")
            .field("timer_overflow_int_st", &self.timer_overflow_int_st())
            .finish()
    }
}
#[doc = "TWAIFD interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_INT_ST_SPEC;
impl crate::RegisterSpec for TIMER_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_int_st::R`](R) reader structure"]
impl crate::Readable for TIMER_INT_ST_SPEC {}
#[doc = "`reset()` method sets TIMER_INT_ST to value 0"]
impl crate::Resettable for TIMER_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
