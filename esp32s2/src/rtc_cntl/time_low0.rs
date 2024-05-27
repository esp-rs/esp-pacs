///Register `TIME_LOW0` reader
pub type R = crate::R<TIME_LOW0_SPEC>;
///Field `TIMER_VALUE0_LOW` reader - Stores the lower 32 bits of RTC timer 0.
pub type TIMER_VALUE0_LOW_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the lower 32 bits of RTC timer 0.
    #[inline(always)]
    pub fn timer_value0_low(&self) -> TIMER_VALUE0_LOW_R {
        TIMER_VALUE0_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_LOW0")
            .field("timer_value0_low", &self.timer_value0_low())
            .finish()
    }
}
/**Stores the lower 32 bits of RTC timer 0.

You can [`read`](crate::generic::Reg::read) this register and get [`time_low0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIME_LOW0_SPEC;
impl crate::RegisterSpec for TIME_LOW0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`time_low0::R`](R) reader structure
impl crate::Readable for TIME_LOW0_SPEC {}
///`reset()` method sets TIME_LOW0 to value 0
impl crate::Resettable for TIME_LOW0_SPEC {
    const RESET_VALUE: u32 = 0;
}
