#[doc = "Register `TIMER_INT_RAW` reader"]
pub type R = crate::R<TIMER_INT_RAW_SPEC>;
#[doc = "Field `TIMER_OVERFLOW_INT_RAW` reader - The raw bit signal for read_done interrupt."]
pub type TIMER_OVERFLOW_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw bit signal for read_done interrupt."]
    #[inline(always)]
    pub fn timer_overflow_int_raw(&self) -> TIMER_OVERFLOW_INT_RAW_R {
        TIMER_OVERFLOW_INT_RAW_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_INT_RAW")
            .field("timer_overflow_int_raw", &self.timer_overflow_int_raw())
            .finish()
    }
}
#[doc = "TWAIFD raw interrupt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_INT_RAW_SPEC;
impl crate::RegisterSpec for TIMER_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_int_raw::R`](R) reader structure"]
impl crate::Readable for TIMER_INT_RAW_SPEC {}
#[doc = "`reset()` method sets TIMER_INT_RAW to value 0"]
impl crate::Resettable for TIMER_INT_RAW_SPEC {}
